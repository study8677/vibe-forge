from __future__ import annotations

import json
import re
from dataclasses import asdict, is_dataclass
from datetime import datetime
from pathlib import Path
from typing import Any

from research_agent.models import SessionReport


def _slugify(value: str) -> str:
    collapsed = re.sub(r"[^a-zA-Z0-9]+", "-", value).strip("-").lower()
    return collapsed[:48] or "research-run"


def _to_jsonable(value: Any) -> Any:
    if is_dataclass(value):
        return _to_jsonable(asdict(value))
    if isinstance(value, Path):
        return str(value)
    if isinstance(value, dict):
        return {key: _to_jsonable(item) for key, item in value.items()}
    if isinstance(value, list):
        return [_to_jsonable(item) for item in value]
    return value


class ResearchWorkspace:
    def __init__(self, runs_root: Path, goal: str):
        self.runs_root = runs_root
        self.runs_root.mkdir(parents=True, exist_ok=True)
        timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
        self.run_dir = self.runs_root / f"{timestamp}-{_slugify(goal)}"
        self.run_dir.mkdir(parents=True, exist_ok=True)

    def cycle_dir(self, cycle_index: int) -> Path:
        path = self.run_dir / f"cycle_{cycle_index:02d}"
        path.mkdir(parents=True, exist_ok=True)
        return path

    def write_text(self, path: Path, content: str) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")

    def write_json(self, path: Path, payload: Any) -> None:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(
            json.dumps(_to_jsonable(payload), ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    def write_code_attempt(self, cycle_index: int, attempt_index: int, code: str) -> Path:
        path = self.cycle_dir(cycle_index) / f"attempt_{attempt_index:02d}.py"
        self.write_text(path, code)
        return path

    def finalize(self, report: SessionReport) -> tuple[Path, Path]:
        summary_json = self.run_dir / "session_summary.json"
        final_report = self.run_dir / "final_report.md"
        self.write_json(summary_json, report)
        self.write_text(final_report, self._build_markdown_report(report))
        return final_report, summary_json

    def _build_markdown_report(self, report: SessionReport) -> str:
        lines = [
            "# Research Agent Report",
            "",
            f"Goal: {report.goal}",
            f"Completed cycles: {report.completed_cycles}",
            "",
        ]
        for cycle in report.cycles:
            lines.extend(
                [
                    f"## Cycle {cycle.cycle_index}",
                    "",
                    f"Idea: {cycle.idea.title}",
                    f"Execution success: {cycle.execution.success}",
                    f"Repair attempts: {cycle.repair_attempts}",
                    f"Evaluation summary: {cycle.evaluation.summary}",
                    "",
                    "Top paper takeaways:",
                ]
            )
            for digest in cycle.digests:
                lines.append(f"- {digest.paper_id}: {digest.takeaway}")
            lines.extend(
                [
                    "",
                    "Strengths:",
                ]
            )
            for strength in cycle.evaluation.strengths:
                lines.append(f"- {strength}")
            if cycle.evaluation.limitations:
                lines.extend(["", "Limitations:"])
                for limitation in cycle.evaluation.limitations:
                    lines.append(f"- {limitation}")
            lines.append("")
        return "\n".join(lines)
