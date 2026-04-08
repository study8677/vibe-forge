from __future__ import annotations

from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

from research_agent.executor import PythonExecutor
from research_agent.llm.base import ResearchLLM
from research_agent.models import CodeAttempt, CycleReport, SessionReport
from research_agent.workspace import ResearchWorkspace


@dataclass(slots=True)
class ResearchSettings:
    max_cycles: int = 2
    max_papers: int = 8
    max_ranked_papers: int = 5
    max_repair_attempts: int = 2
    execution_timeout_seconds: int = 60


class ResearchOrchestrator:
    def __init__(
        self,
        *,
        settings: ResearchSettings,
        search_client: Any,
        llm: ResearchLLM,
        runs_root: Path = Path("runs"),
        executor: PythonExecutor | None = None,
    ):
        self.settings = settings
        self.search_client = search_client
        self.llm = llm
        self.runs_root = runs_root
        self.executor = executor or PythonExecutor(
            timeout_seconds=settings.execution_timeout_seconds
        )

    def run(self, goal: str) -> SessionReport:
        workspace = ResearchWorkspace(self.runs_root, goal)
        cycle_reports: list[CycleReport] = []
        for cycle_index in range(1, self.settings.max_cycles + 1):
            papers = self.search_client.search(goal, self.settings.max_papers)
            ranked_papers = self.llm.rank_papers(goal, papers)[: self.settings.max_ranked_papers]
            digests = self.llm.digest_papers(goal, ranked_papers)
            prior_cycles_payload = [self._cycle_payload(cycle) for cycle in cycle_reports]
            idea = self.llm.generate_idea(goal, digests, prior_cycles_payload)

            attempts: list[CodeAttempt] = []
            code = self.llm.generate_code(goal, idea, digests, self._attempt_payloads(attempts))
            execution = None
            code_path = None
            repair_attempts = 0

            for attempt_index in range(1, self.settings.max_repair_attempts + 2):
                code_path = workspace.write_code_attempt(cycle_index, attempt_index, code)
                execution = self.executor.run(code_path)
                attempts.append(
                    CodeAttempt(
                        attempt_index=attempt_index,
                        code_path=code_path,
                        code=code,
                        execution=execution,
                    )
                )
                execution_path = (
                    workspace.cycle_dir(cycle_index)
                    / f"attempt_{attempt_index:02d}_execution.json"
                )
                workspace.write_json(
                    execution_path,
                    execution,
                )
                if execution.success:
                    break
                if attempt_index > self.settings.max_repair_attempts:
                    break
                repair_attempts += 1
                code = self.llm.repair_code(
                    goal,
                    idea,
                    code,
                    self._execution_context(execution),
                    self._attempt_payloads(attempts),
                )

            assert execution is not None
            assert code_path is not None

            evaluation = self.llm.evaluate_cycle(
                goal,
                idea,
                asdict(execution),
                digests,
                prior_cycles_payload,
            )
            cycle_dir = workspace.cycle_dir(cycle_index)
            workspace.write_json(cycle_dir / "papers.json", papers)
            workspace.write_json(cycle_dir / "ranked_papers.json", ranked_papers)
            workspace.write_json(cycle_dir / "digests.json", digests)
            workspace.write_json(cycle_dir / "idea.json", idea)
            workspace.write_json(cycle_dir / "evaluation.json", evaluation)

            cycle_report = CycleReport(
                cycle_index=cycle_index,
                papers=papers,
                ranked_papers=ranked_papers,
                digests=digests,
                idea=idea,
                attempts=attempts,
                execution=execution,
                evaluation=evaluation,
                repair_attempts=repair_attempts,
                code_path=code_path,
                cycle_dir=cycle_dir,
            )
            cycle_reports.append(cycle_report)

            if evaluation.recommendation == "stop":
                break

        placeholder_report = SessionReport(
            goal=goal,
            run_dir=workspace.run_dir,
            cycles=cycle_reports,
            final_report_path=workspace.run_dir / "final_report.md",
            summary_json_path=workspace.run_dir / "session_summary.json",
            completed_cycles=len(cycle_reports),
        )
        final_report_path, summary_json_path = workspace.finalize(placeholder_report)
        return SessionReport(
            goal=goal,
            run_dir=workspace.run_dir,
            cycles=cycle_reports,
            final_report_path=final_report_path,
            summary_json_path=summary_json_path,
            completed_cycles=len(cycle_reports),
        )

    @staticmethod
    def _execution_context(execution: Any) -> str:
        stdout = execution.stdout.strip()
        stderr = execution.stderr.strip()
        return f"stdout:\n{stdout}\n\nstderr:\n{stderr}".strip()

    @staticmethod
    def _attempt_payloads(attempts: list[CodeAttempt]) -> list[dict[str, Any]]:
        return [
            {
                "attempt_index": attempt.attempt_index,
                "code_path": str(attempt.code_path),
                "success": attempt.execution.success,
                "returncode": attempt.execution.returncode,
                "stderr": attempt.execution.stderr,
            }
            for attempt in attempts
        ]

    @staticmethod
    def _cycle_payload(cycle: CycleReport) -> dict[str, Any]:
        return {
            "cycle_index": cycle.cycle_index,
            "idea": asdict(cycle.idea),
            "evaluation": asdict(cycle.evaluation),
            "execution_success": cycle.execution.success,
        }
