from __future__ import annotations

import subprocess
import sys
import time
from pathlib import Path

from research_agent.models import ExecutionResult


class PythonExecutor:
    def __init__(self, timeout_seconds: int = 60):
        self.timeout_seconds = timeout_seconds

    def run(self, script_path: Path) -> ExecutionResult:
        started = time.monotonic()
        try:
            completed = subprocess.run(
                [sys.executable, script_path.name],
                cwd=script_path.parent,
                capture_output=True,
                text=True,
                timeout=self.timeout_seconds,
                check=False,
            )
            duration = time.monotonic() - started
            return ExecutionResult(
                success=completed.returncode == 0,
                returncode=completed.returncode,
                stdout=completed.stdout,
                stderr=completed.stderr,
                duration_seconds=duration,
            )
        except subprocess.TimeoutExpired as exc:
            duration = time.monotonic() - started
            return ExecutionResult(
                success=False,
                returncode=124,
                stdout=exc.stdout or "",
                stderr=(exc.stderr or "") + "\nExecution timed out.",
                duration_seconds=duration,
            )
