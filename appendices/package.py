"""Build native teaching applications and create a non-overwriting ZIP."""

from __future__ import annotations

import argparse
import hashlib
import json
import platform
import re
import subprocess
import tomllib
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True)
    version = parser.parse_args().version
    actual = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))["workspace"]["package"]["version"]
    if version != actual or not re.fullmatch(r"\d+\.\d+\.\d+(?:-[A-Za-z0-9.-]+)?", version):
        parser.error("Use the workspace package version; update it before releasing a new version")
    if platform.system() not in {"Windows", "Linux"}:
        parser.error("This packaging recipe supports native Windows and Linux builds")
    machine = platform.machine().lower().replace("amd64", "x86_64")
    name = f"rust-appendix-apps-{version}-{platform.system().lower()}-{machine}.zip"
    directory = ROOT / "target" / "packages"
    directory.mkdir(parents=True, exist_ok=True)
    destination = directory / name
    if destination.exists() or destination.with_suffix(".zip.sha256").exists():
        parser.error(f"Refusing to overwrite an existing package: {destination}")
    compiler = subprocess.check_output(["rustc", "--version", "--verbose"], cwd=ROOT, text=True)
    host = next(line.removeprefix("host: ") for line in compiler.splitlines() if line.startswith("host: "))
    subprocess.run(["cargo", "build", "-p", "inventory-cli", "-p", "sum-service",
                    "--bins", "--release", "--locked", "--target", host,
                    "--target-dir", str(ROOT / "target")], cwd=ROOT, check=True)
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1", "--locked", "--filter-platform", host],
        cwd=ROOT, text=True, encoding="utf-8"))
    # Walk application runtime/build dependencies, excluding unrelated labs and
    # dev dependencies. Collect actual files, not just SPDX expressions.
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    pending = [package["id"] for package in metadata["packages"]
               if package["source"] is None and package["name"] in {"inventory-cli", "sum-service"}]
    included = set()
    while pending:
        identifier = pending.pop()
        if identifier in included:
            continue
        included.add(identifier)
        pending.extend(dependency["pkg"] for dependency in nodes[identifier]["deps"]
                       if any(kind["kind"] != "dev" for kind in dependency["dep_kinds"]))
    notices = []
    for package in metadata["packages"]:
        if package["source"] is None or package["id"] not in included:
            continue
        parent = Path(package["manifest_path"]).parent
        license_files = sorted(path for path in parent.iterdir() if path.is_file()
                               and path.name.upper().startswith(("LICENSE", "COPYING", "COPYRIGHT")))
        if package.get("license_file"):
            declared = parent / package["license_file"]
            if declared not in license_files:
                license_files.append(declared)
        if not license_files:
            raise RuntimeError(f"Review dependency license files before packaging: {package['name']}")
        notices.append(f"\n=== {package['name']} {package['version']} ({package.get('license')}) ===\n")
        for path in license_files:
            notices.append(f"\n--- {path.name} ---\n{path.read_text(encoding='utf-8')}\n")
    suffix = ".exe" if platform.system() == "Windows" else ""
    binaries = [ROOT / "target" / host / "release" / f"{name}{suffix}"
                for name in ("inventory-cli", "sum-server", "sum-client")]
    for binary in binaries:
        subprocess.run([str(binary), "--help"], cwd=ROOT, check=True, capture_output=True)
    build_info = {"version": version, "platform": platform.platform(),
                  "rustc": compiler,
                  "cargo_lock_sha256": hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest(),
                  "binaries_sha256": {path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in binaries},
                  "scope": "Native teaching build; not signed, not installed or tested on a clean machine"}
    with zipfile.ZipFile(destination, "x", compression=zipfile.ZIP_DEFLATED) as archive:
        for binary in binaries:
            archive.write(binary, binary.name)
        archive.write(ROOT / "LICENSE", "LICENSE")
        archive.write(ROOT / "Cargo.lock", "Cargo.lock")
        for package in ("inventory-cli", "sum-service"):
            archive.write(ROOT / package / "README.md", f"{package}/README.md")
        archive.write(ROOT / "inventory-cli" / "fixtures" / "inventory.csv", "inventory-cli/fixtures/inventory.csv")
        archive.writestr("THIRD-PARTY-NOTICES.txt", "".join(notices))
        archive.writestr("BUILD-INFO.json", json.dumps(build_info, indent=2) + "\n")
    checksum = hashlib.sha256(destination.read_bytes()).hexdigest()
    with destination.with_suffix(".zip.sha256").open("x", encoding="utf-8") as checksum_file:
        checksum_file.write(f"{checksum}  {name}\n")
    print(destination)


if __name__ == "__main__":
    main()
