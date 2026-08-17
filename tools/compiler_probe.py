#!/usr/bin/env python3
"""Verifica el laboratorio estable de MIR, LLVM IR y assembly del capítulo 52."""

from __future__ import annotations

import json
import os
import subprocess
import tempfile
from pathlib import Path


CODE_ROOT = Path(__file__).resolve().parents[1]
SOURCE = CODE_ROOT / "compiler_lab" / "pipeline.rs"


def run(command: list[str], cwd: Path) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        command,
        cwd=cwd,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        timeout=120,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"Falló {' '.join(command)}\n{completed.stdout}\n{completed.stderr}"
        )
    return completed


def one_file(directory: Path, suffix: str) -> Path:
    matches = list(directory.glob(f"*{suffix}"))
    if len(matches) != 1:
        raise RuntimeError(
            f"Se esperaba un artefacto {suffix} y se encontraron {len(matches)}"
        )
    return matches[0]


def emit(directory: Path, label: str, opt_level: int) -> dict[str, int]:
    output = directory / label
    output.mkdir()
    run(
        [
            "rustc",
            "--edition=2024",
            "--crate-name",
            f"c52_{label}",
            "--crate-type=lib",
            f"-Copt-level={opt_level}",
            "--emit=mir,llvm-ir,asm",
            "--out-dir",
            str(output),
            str(SOURCE),
        ],
        CODE_ROOT,
    )

    mir = one_file(output, ".mir")
    llvm_ir = one_file(output, ".ll")
    assembly = one_file(output, ".s")
    mir_text = mir.read_text(encoding="utf-8", errors="replace")
    llvm_text = llvm_ir.read_text(encoding="utf-8", errors="replace")
    assembly_text = assembly.read_text(encoding="utf-8", errors="replace")

    required = {
        "MIR": ("inspect_ticket", "drop("),
        "LLVM IR": ("c52_exported_add", "c52_twice_u64"),
        "assembly": ("c52_exported_add", "c52_twice_u64"),
    }
    observed = {
        "MIR": mir_text,
        "LLVM IR": llvm_text,
        "assembly": assembly_text,
    }
    for artifact, markers in required.items():
        missing = [marker for marker in markers if marker not in observed[artifact]]
        if missing:
            raise RuntimeError(f"{artifact} no contiene los marcadores {missing}")

    return {
        "mir_bytes": mir.stat().st_size,
        "llvm_ir_bytes": llvm_ir.stat().st_size,
        "assembly_bytes": assembly.stat().st_size,
    }


def main() -> None:
    if not SOURCE.is_file():
        raise RuntimeError(f"No existe la fuente del laboratorio: {SOURCE}")

    with tempfile.TemporaryDirectory(prefix="rust-c52-compiler-probe-") as temporary:
        directory = Path(temporary)
        executable = directory / ("c52-tests.exe" if os.name == "nt" else "c52-tests")
        run(
            [
                "rustc",
                "--edition=2024",
                "--test",
                str(SOURCE),
                "-o",
                str(executable),
            ],
            CODE_ROOT,
        )
        runtime = run([str(executable)], CODE_ROOT)
        result = {
            "ok": True,
            "runtime_test_passed": "1 passed" in runtime.stdout,
            "debug": emit(directory, "debug", 0),
            "optimized": emit(directory, "optimized", 3),
        }
        if not result["runtime_test_passed"]:
            raise RuntimeError("El binario de test no confirmó el test runtime")
        print(json.dumps(result, ensure_ascii=False, indent=2))


if __name__ == "__main__":
    main()
