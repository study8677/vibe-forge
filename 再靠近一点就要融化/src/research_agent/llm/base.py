from __future__ import annotations

from typing import Protocol

from research_agent.models import EvaluationResult, ExperimentIdea, Paper, PaperDigest, RankedPaper


class ResearchLLM(Protocol):
    def rank_papers(self, goal: str, papers: list[Paper]) -> list[RankedPaper]:
        ...

    def digest_papers(self, goal: str, ranked_papers: list[RankedPaper]) -> list[PaperDigest]:
        ...

    def generate_idea(
        self,
        goal: str,
        digests: list[PaperDigest],
        previous_cycles: list[object],
    ) -> ExperimentIdea:
        ...

    def generate_code(
        self,
        goal: str,
        idea: ExperimentIdea,
        digests: list[PaperDigest],
        previous_attempts: list[object],
    ) -> str:
        ...

    def repair_code(
        self,
        goal: str,
        idea: ExperimentIdea,
        broken_code: str,
        error_context: str,
        previous_attempts: list[object],
    ) -> str:
        ...

    def evaluate_cycle(
        self,
        goal: str,
        idea: ExperimentIdea,
        execution_result: object,
        digests: list[PaperDigest],
        previous_cycles: list[object],
    ) -> EvaluationResult:
        ...
