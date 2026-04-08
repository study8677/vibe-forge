import tempfile
import unittest
from pathlib import Path

from research_agent.models import EvaluationResult, ExperimentIdea, Paper, PaperDigest, RankedPaper
from research_agent.orchestrator import ResearchOrchestrator, ResearchSettings


class FakeSearch:
    def search(self, goal: str, limit: int) -> list[Paper]:
        del goal, limit
        return [
            Paper(
                paper_id="1234.5678v1",
                title="Sample Paper",
                summary="Abstract",
                authors=["Alice"],
                pdf_url="http://arxiv.org/pdf/1234.5678v1",
                published="2026-04-01T00:00:00Z",
            )
        ]


class FakeLLM:
    def __init__(self) -> None:
        self.repair_calls = 0
        self.generated_codes = 0

    def rank_papers(self, goal: str, papers: list[Paper]) -> list[RankedPaper]:
        del goal
        return [RankedPaper(paper=papers[0], relevance_score=0.95, rationale="Exact topic match")]

    def digest_papers(self, goal: str, ranked_papers: list[RankedPaper]) -> list[PaperDigest]:
        del goal
        return [
            PaperDigest(
                paper_id=ranked_papers[0].paper.paper_id,
                takeaway="Useful baseline",
                relevance="High",
            )
        ]

    def generate_idea(
        self,
        goal: str,
        digests: list[PaperDigest],
        previous_cycles: list[object],
    ) -> ExperimentIdea:
        del goal, digests, previous_cycles
        return ExperimentIdea(
            title="Baseline experiment",
            hypothesis="A baseline script should run",
            method="Print a success marker",
            success_metric="script exits successfully",
        )

    def generate_code(
        self,
        goal: str,
        idea: ExperimentIdea,
        digests: list[PaperDigest],
        previous_attempts: list[object],
    ) -> str:
        del goal, idea, digests, previous_attempts
        self.generated_codes += 1
        return "raise RuntimeError('boom')\n"

    def repair_code(
        self,
        goal: str,
        idea: ExperimentIdea,
        broken_code: str,
        error_context: str,
        previous_attempts: list[object],
    ) -> str:
        del goal, idea, broken_code, error_context, previous_attempts
        self.repair_calls += 1
        return "print('success')\n"

    def evaluate_cycle(
        self,
        goal: str,
        idea: ExperimentIdea,
        execution_result: object,
        digests: list[PaperDigest],
        previous_cycles: list[object],
    ) -> EvaluationResult:
        del goal, idea, execution_result, digests, previous_cycles
        return EvaluationResult(
            summary="Execution succeeded",
            strengths=["Recovered from failure"],
            limitations=[],
            confidence=0.8,
            recommendation="stop",
            next_step="",
        )


class ResearchLoopTest(unittest.TestCase):
    def test_research_loop_repairs_code_until_success(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            orchestrator = ResearchOrchestrator(
                settings=ResearchSettings(max_cycles=1, max_papers=3, max_repair_attempts=2),
                search_client=FakeSearch(),
                llm=FakeLLM(),
                runs_root=Path(tmpdir),
            )

            report = orchestrator.run("test repair loop")

            self.assertEqual(report.completed_cycles, 1)
            self.assertTrue(report.cycles[0].execution.success)
            self.assertEqual(report.cycles[0].repair_attempts, 1)
            self.assertTrue(report.final_report_path.exists())
