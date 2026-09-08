"""Verify the supplementary workspace without regenerating the main corpus."""

from __future__ import annotations

import argparse
import hashlib
import json
import platform
import re
import subprocess
import sys
import time
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent
DOCS = ROOT.parent.parent / "docs"
MANIFEST = ROOT / "listings.json"


def digest(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def extract() -> tuple[list[dict[str, object]], dict[str, str]]:
    entries = []
    corpora = {"lab/doctests.md": ["# Supplementary appendix examples", ""],
               "portable-core/doctests.md": ["# Portable appendix examples", ""]}
    for path in sorted(DOCS.glob("*.Apendice-*.md")):
        letter = re.search(r"Apendice-([A-Z])-", path.name).group(1)
        tests = corpora["portable-core/doctests.md" if letter == "G" else "lab/doctests.md"]
        lines = path.read_text(encoding="utf-8").splitlines()
        opening = None
        count = 0
        for index, line in enumerate(lines):
            if not line.startswith("```"):
                continue
            if opening is None:
                opening = index
                continue
            tag = lines[opening][3:].strip()
            body = "\n".join(lines[opening + 1:index]) + "\n"
            count += 1
            identifier = f"AP-{letter}-B{count:03d}"
            language = tag.split(",")[0]
            if language == "toml":
                tomllib.loads(body)
            if language == "json":
                json.loads(body)
            entry = {"id": identifier, "source": path.name, "line": opening + 1,
                     "tag": tag, "sha256": digest(body), "body": body}
            entries.append(entry)
            if language == "rust":
                tests.extend([f"## {identifier}", "", f"```{tag}", body.rstrip(), "```", ""])
            opening = None
        if opening is not None:
            raise ValueError(f"Unclosed fence: {path.name}:{opening + 1}")
    return entries, {path: "\n".join(lines) for path, lines in corpora.items()}


def run(args: list[str]) -> dict[str, object]:
    print("Running: " + " ".join(args), flush=True)
    started = time.monotonic()
    result = subprocess.run(args, cwd=ROOT, capture_output=True, text=True,
                            encoding="utf-8", errors="replace", timeout=900)
    if result.returncode:
        print(result.stdout + result.stderr, flush=True)
    return {"command": args, "returncode": result.returncode,
            "seconds": round(time.monotonic() - started, 2),
            "stdout": result.stdout, "stderr": result.stderr}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sync", action="store_true")
    parser.add_argument("--check-docs", action="store_true")
    options = parser.parse_args()
    if options.sync or options.check_docs:
        if not DOCS.is_dir():
            parser.error("The manuscript is required for --sync/--check-docs")
        entries, corpora = extract()
        manifest = {"schema": 2, "corpora": {path: digest(tests) for path, tests in corpora.items()},
                    "listings": entries}
        if options.sync:
            for path, tests in corpora.items():
                (ROOT / path).write_text(tests, encoding="utf-8", newline="\n")
            MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n",
                                encoding="utf-8", newline="\n")
        elif (json.loads(MANIFEST.read_text(encoding="utf-8")) != manifest
              or any((ROOT / path).read_text(encoding="utf-8") != tests for path, tests in corpora.items())):
            raise ValueError("Appendix listings are stale; run verify.py --sync")
        if options.check_docs:
            print(f"Appendix listings match the manuscript: {len(entries)}")
            return 0
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    for path, expected in manifest["corpora"].items():
        if digest((ROOT / path).read_text(encoding="utf-8")) != expected:
            raise ValueError(f"The committed doctest corpus was changed independently: {path}")
    for listing in manifest["listings"]:
        if digest(listing["body"]) != listing["sha256"]:
            raise ValueError(f"Changed listing: {listing['id']}")
    commands = [
        ["cargo", "fmt", "--all", "--check"],
        ["cargo", "test", "--workspace", "--all-targets", "--locked"],
        ["cargo", "test", "--workspace", "--doc", "--locked"],
        ["cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"],
        ["cargo", "test", "--workspace", "--release", "--locked"],
        ["cargo", "test", "-p", "portable-core", "--features", "alloc", "--locked"],
        ["cargo", "clippy", "-p", "portable-core", "--all-targets", "--all-features", "--locked", "--", "-D", "warnings"],
        ["cargo", "fmt", "--manifest-path", "fuzz/Cargo.toml", "--check"],
        ["cargo", "check", "--manifest-path", "fuzz/Cargo.toml", "--locked"],
    ]
    checks = [run(args) for args in commands]
    report = {"platform": platform.platform(), "python": sys.version,
              "rustc": subprocess.check_output(["rustc", "--version"], cwd=ROOT, text=True).strip(),
              "listings": len(manifest["listings"]),
              "rust_blocks": sum(item["tag"].split(",")[0] == "rust" for item in manifest["listings"]),
              "checks": checks, "ok": all(check["returncode"] == 0 for check in checks),
              "not_run_by_this_command": ["OS installation", "Zed GUI", "Miri", "fuzzing", "cross targets"]}
    reports = ROOT / "reports"
    reports.mkdir(exist_ok=True)
    (reports / "verification.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({key: value for key, value in report.items() if key != "checks"}, indent=2))
    return 0 if report["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
