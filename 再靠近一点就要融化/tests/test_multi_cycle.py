import tempfile
import unittest
from pathlib import Path

from research_agent.models import EvaluationResult
from research_agent.orchestrator import ResearchOrchestrator, ResearchSettings
from tests.test_research_loop import FakeLLM, FakeSearch


class MultiCycleLLM(FakeLLM):
    def __init__(self) -> None:
        super().__init__()
        self.eval_calls = 0

    def evaluate_cycle(self, goal, idea, execution_result, digests, previous_cycles):
        del goal, idea, execution_result, digests, previous_cycles
        self.eval_calls += 1
        recommendation = "continue" if self.eval_calls == 1 else "stop"
        return EvaluationResult(
            summary="Cycle complete",
            strengths=["Produced an output"],
            limitations=[],
            confidence=0.7,
            recommendation=recommendation,
            next_step="Try a refined variant" if recommendation == "continue" else "",
        )


class MultiCycleTest(unittest.TestCase):
    def test_orchestrator_runs_multiple_cycles_when_requested(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            orchestrator = ResearchOrchestrator(
                settings=ResearchSettings(max_cycles=2, max_papers=3, max_repair_attempts=1),
                search_client=FakeSearch(),
                llm=MultiCycleLLM(),
                runs_root=Path(tmpdir),
            )

            report = orchestrator.run("test multi cycle")

            self.assertEqual(report.completed_cycles, 2)
