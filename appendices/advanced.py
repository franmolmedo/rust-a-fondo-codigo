"""Run explicitly selected advanced labs and record positive/negative controls."""

from __future__ import annotations

import argparse
import json
import platform
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent
NIGHTLY = "nightly-2026-04-15"


def check(command: list[str], expected_diagnostic: str | None = None) -> dict:
    print("Running: " + " ".join(command), flush=True)
    started = time.monotonic()
    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True,
                            encoding="utf-8", errors="replace", timeout=900)
    output = result.stdout + result.stderr
    ok = (result.returncode == 0 if expected_diagnostic is None
          else result.returncode != 0 and expected_diagnostic in output)
    print(output, flush=True)
    return {"command": command, "exit_code": result.returncode,
            "expected_diagnostic": expected_diagnostic, "ok": ok,
            "seconds": round(time.monotonic() - started, 2), "output": output}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--miri", action="store_true")
    parser.add_argument("--fuzz", action="store_true")
    parser.add_argument("--cross", action="store_true")
    options = parser.parse_args()
    if not (options.miri or options.fuzz or options.cross):
        parser.error("Select --miri, --fuzz or --cross; toolchains/targets must be installed first")
    if options.fuzz and platform.system() != "Linux":
        parser.error("This scripted sanitizer recipe targets Linux; consult the book for other hosts")
    checks = []
    if options.miri:
        checks.append(check(["cargo", f"+{NIGHTLY}", "miri", "setup"]))
        if checks[-1]["ok"]:
            checks.append(check(["cargo", f"+{NIGHTLY}", "miri", "test", "-p",
                                 "rust-appendix-lab", "--lib", "--locked", "audits::tests"]))
            checks.append(check(["cargo", f"+{NIGHTLY}", "miri", "run", "-p",
                                 "rust-appendix-lab", "--example", "miri_bad_read", "--locked"],
                                "Undefined Behavior"))
    if options.fuzz:
        checks.append(check(["cargo", f"+{NIGHTLY}", "fuzz", "run", "parse_record", "--",
                             "-max_total_time=30", "-max_len=1024"]))
    if options.cross:
        for features in ([], ["--features", "alloc"]):
            checks.append(check(["cargo", "+1.95.0", "build", "-p", "portable-core", "--lib",
                                 "--target", "thumbv7em-none-eabihf", "--locked", *features]))
    report = {"platform": platform.platform(), "nightly": NIGHTLY, "checks": checks,
              "ok": bool(checks) and all(item["ok"] for item in checks)}
    directory = ROOT / "reports"
    directory.mkdir(exist_ok=True)
    name = "advanced-" + "-".join(key for key in ("miri", "fuzz", "cross") if getattr(options, key))
    (directory / f"{name}.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(f"Advanced checks passed: {report['ok']}")
    return 0 if report["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
