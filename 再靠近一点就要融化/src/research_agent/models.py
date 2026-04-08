from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Literal


@dataclass(slots=True)
class Paper:
    paper_id: str
    title: str
    summary: str
    authors: list[str]
    pdf_url: str
    published: str


@dataclass(slots=True)
class RankedPaper:
    paper: Paper
    relevance_score: float
    rationale: str


@dataclass(slots=True)
class PaperDigest:
    paper_id: str
    takeaway: str
    relevance: str


@dataclass(slots=True)
class ExperimentIdea:
    title: str
    hypothesis: str
    method: str
    success_metric: str
    risks: list[str] = field(default_factory=list)


@dataclass(slots=True)
class ExecutionResult:
    success: bool
    returncode: int
    stdout: str
    stderr: str
    duration_seconds: float = 0.0


@dataclass(slots=True)
class EvaluationResult:
    summary: str
    strengths: list[str]
    limitations: list[str]
    confidence: float
    recommendation: Literal["stop", "continue"]
    next_step: str


@dataclass(slots=True)
class CodeAttempt:
    attempt_index: int
    code_path: Path
    code: str
    execution: ExecutionResult


@dataclass(slots=True)
class CycleReport:
    cycle_index: int
    papers: list[Paper]
    ranked_papers: list[RankedPaper]
    digests: list[PaperDigest]
    idea: ExperimentIdea
    attempts: list[CodeAttempt]
    execution: ExecutionResult
    evaluation: EvaluationResult
    repair_attempts: int
    code_path: Path
    cycle_dir: Path


@dataclass(slots=True)
class SessionReport:
    goal: str
    run_dir: Path
    cycles: list[CycleReport]
    final_report_path: Path
    summary_json_path: Path
    completed_cycles: int
