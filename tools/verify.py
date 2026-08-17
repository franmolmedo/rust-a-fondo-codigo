#!/usr/bin/env python3
"""Audita de forma reproducible el corpus público de Rust a fondo."""

from __future__ import annotations

import hashlib
import json
import re
import subprocess
import sys
import time
import tomllib
from collections import Counter
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
MANIFEST_PATH = REPOSITORY_ROOT / "manifest.json"
REPORT_JSON = REPOSITORY_ROOT / "verification.json"
REPORT_MD = REPOSITORY_ROOT / "VERIFICATION.md"
SOLUTIONS_ROOT = REPOSITORY_ROOT / "solutions"

EXPECTED_LISTINGS = 891
EXPECTED_SOLUTIONS = 403
EXPECTED_SOLUTION_TESTS = 447
SOLUTION_MARKER = re.compile(
    r"(?m)^\s*//\s*SOLUTION:\s*(C\d{2}-[EKPM]\d{2})\s*$"
)
TEST_ATTRIBUTE = re.compile(r"#\[(?:tokio::)?test(?:\([^]]*\))?\]")


def run_command(args: list[str], timeout: int = 1200) -> dict[str, object]:
    """Ejecuta un comando desde la raíz y conserva un resultado serializable."""

    started = time.monotonic()
    try:
        process = subprocess.run(
            args,
            cwd=REPOSITORY_ROOT,
            capture_output=True,
            text=True,
            encoding="utf-8",
            errors="replace",
            timeout=timeout,
        )
        return {
            "command": " ".join(args),
            "ok": process.returncode == 0,
            "returncode": process.returncode,
            "seconds": round(time.monotonic() - started, 2),
            "stdout": process.stdout,
            "stderr": process.stderr,
        }
    except (FileNotFoundError, subprocess.TimeoutExpired) as error:
        return {
            "command": " ".join(args),
            "ok": False,
            "returncode": None,
            "seconds": round(time.monotonic() - started, 2),
            "stdout": "",
            "stderr": str(error),
        }


def load_manifest(failures: list[str]) -> list[dict[str, object]]:
    try:
        data = json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        failures.append(f"manifest.json no se puede leer: {error}")
        return []

    listings = data.get("listings")
    if not isinstance(listings, list):
        failures.append("manifest.json no contiene una lista 'listings'")
        return []
    return listings


def audit_listings(
    listings: list[dict[str, object]], failures: list[str]
) -> tuple[Counter[str], Counter[str], list[dict[str, object]]]:
    identifiers: set[str] = set()
    expected_paths: set[str] = set()
    modes: Counter[str] = Counter()
    languages: Counter[str] = Counter()
    toml_results: list[dict[str, object]] = []

    if len(listings) != EXPECTED_LISTINGS:
        failures.append(
            f"número de listados inesperado: {len(listings)} != {EXPECTED_LISTINGS}"
        )

    for item in listings:
        identifier = str(item.get("id", ""))
        relative_path = str(item.get("path", ""))
        language = str(item.get("language", ""))
        mode = str(item.get("mode", ""))
        expected_hash = str(item.get("sha256", ""))

        if not identifier or identifier in identifiers:
            failures.append(f"identificador ausente o duplicado: {identifier!r}")
        identifiers.add(identifier)

        normalized_path = Path(relative_path).as_posix()
        if not relative_path or normalized_path in expected_paths:
            failures.append(f"ruta ausente o duplicada: {relative_path!r}")
        expected_paths.add(normalized_path)

        path = REPOSITORY_ROOT / relative_path
        if not path.is_file():
            failures.append(f"falta el listado: {relative_path}")
            continue

        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        if digest != expected_hash:
            failures.append(f"hash distinto en: {relative_path}")

        modes[mode] += 1
        languages[language] += 1

        if language != "toml":
            continue

        try:
            source = path.read_text(encoding="utf-8")
            documents = [source]
            if mode == "toml_composite":
                documents = [
                    part
                    for part in re.split(
                        r"(?m)(?=^# .+?/Cargo\.toml\s*$)", source
                    )
                    if part.strip()
                ]
            for document in documents:
                tomllib.loads(document)
            toml_results.append(
                {"id": identifier, "ok": True, "documents": len(documents)}
            )
        except (OSError, tomllib.TOMLDecodeError) as error:
            toml_results.append(
                {"id": identifier, "ok": False, "error": str(error)}
            )
            failures.append(f"TOML inválido {identifier}: {error}")

    actual_paths = {
        path.relative_to(REPOSITORY_ROOT).as_posix()
        for path in (REPOSITORY_ROOT / "listings").rglob("*")
        if path.is_file()
    }
    for missing in sorted(expected_paths - actual_paths):
        failures.append(f"listado declarado pero ausente: {missing}")
    for extra in sorted(actual_paths - expected_paths):
        failures.append(f"listado no declarado en manifest.json: {extra}")

    return modes, languages, toml_results


def audit_solutions(failures: list[str]) -> tuple[int, int]:
    identifiers: list[str] = []
    solution_tests = 0

    for path in sorted(SOLUTIONS_ROOT.rglob("*.rs")):
        source = path.read_text(encoding="utf-8")
        identifiers.extend(SOLUTION_MARKER.findall(source))
        solution_tests += len(TEST_ATTRIBUTE.findall(source))

    duplicates = sorted(
        identifier
        for identifier, count in Counter(identifiers).items()
        if count > 1
    )
    if duplicates:
        failures.append(f"soluciones duplicadas: {', '.join(duplicates)}")
    if len(identifiers) != EXPECTED_SOLUTIONS:
        failures.append(
            f"número de soluciones inesperado: {len(identifiers)} != {EXPECTED_SOLUTIONS}"
        )
    if solution_tests != EXPECTED_SOLUTION_TESTS:
        failures.append(
            "número de tests de soluciones inesperado: "
            f"{solution_tests} != {EXPECTED_SOLUTION_TESTS}"
        )

    return len(identifiers), solution_tests


def write_reports(report: dict[str, object]) -> None:
    REPORT_JSON.write_text(
        json.dumps(report, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

    status = "APROBADA" if report["ok"] else "FALLIDA"
    modes = report["modes"]
    languages = report["languages"]
    toml = report["toml"]
    solutions = report["solutions"]
    lines = [
        "# Verificación del corpus",
        "",
        f"**Resultado: {status}.**",
        "",
        f"- Toolchain: `{report['rustc']}`",
        f"- Bloques conservados: **{report['listings']}**",
        "- Bloques Rust verificados por Cargo: "
        f"**{modes.get('run', 0) + modes.get('compile_only', 0) + modes.get('compile_fail', 0) + modes.get('should_panic', 0)}/{languages.get('rust', 0)}**",
        f"- Rust ejecutable: **{modes.get('run', 0)}**",
        f"- Rust de solo compilación: **{modes.get('compile_only', 0)}**",
        f"- Errores esperados: **{modes.get('compile_fail', 0)}**",
        f"- Panics esperados: **{modes.get('should_panic', 0)}**",
        f"- Fragmentos ilustrativos: **{modes.get('illustrative', 0)}**",
        f"- Fragmentos contextuales: **{modes.get('contextual', 0)}**",
        f"- TOML válido: **{toml['passed']}/{toml['checked']}**",
        f"- Soluciones ejecutables de referencia: **{solutions['implemented']}**",
        f"- Tests declarados en la crate de soluciones: **{solutions['tests']}**",
        "",
        "## Comprobaciones",
        "",
    ]
    for item in report["checks"]:
        mark = "OK" if item["ok"] else "ERROR"
        lines.append(
            f"- **{mark}** — `{item['command']}` ({item['seconds']} s)"
        )
    if report["failures"]:
        lines.extend(["", "## Errores", ""])
        lines.extend(f"- {failure}" for failure in report["failures"])

    REPORT_MD.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    failures: list[str] = []
    listings = load_manifest(failures)
    modes, languages, toml_results = audit_listings(listings, failures)
    solution_count, solution_tests = audit_solutions(failures)

    compiler_probe = run_command([sys.executable, "tools/compiler_probe.py"])
    compiler_probe["command"] = "python tools/compiler_probe.py"
    checks = [
        run_command(["cargo", "check", "--workspace", "--locked"]),
        run_command(
            ["cargo", "check", "--workspace", "--all-features", "--locked"]
        ),
        run_command(
            ["cargo", "test", "--workspace", "--all-targets", "--locked"]
        ),
        run_command(
            [
                "cargo",
                "test",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--locked",
            ]
        ),
        run_command(["cargo", "test", "--workspace", "--doc", "--locked"]),
        run_command(
            [
                "cargo",
                "test",
                "--workspace",
                "--doc",
                "--all-features",
                "--locked",
            ]
        ),
        run_command(["cargo", "fmt", "--all", "--check"]),
        run_command(
            [
                "cargo",
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--locked",
                "--",
                "-D",
                "warnings",
            ]
        ),
        compiler_probe,
    ]
    for check in checks:
        if not check["ok"]:
            failures.append(f"comando fallido: {check['command']}")

    rustc = run_command(["rustc", "--version"])
    report: dict[str, object] = {
        "ok": not failures,
        "rustc": str(rustc["stdout"]).strip() if rustc["ok"] else "no disponible",
        "listings": len(listings),
        "languages": dict(sorted(languages.items())),
        "modes": dict(sorted(modes.items())),
        "toml": {
            "checked": len(toml_results),
            "passed": sum(bool(item["ok"]) for item in toml_results),
        },
        "solutions": {
            "implemented": solution_count,
            "tests": solution_tests,
        },
        "checks": [
            {
                "command": item["command"],
                "ok": item["ok"],
                "returncode": item["returncode"],
                "seconds": item["seconds"],
            }
            for item in checks
        ],
        "failures": failures,
    }
    write_reports(report)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    if failures:
        sys.exit(1)


if __name__ == "__main__":
    main()
