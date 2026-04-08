from __future__ import annotations

import argparse
from pathlib import Path

from research_agent.arxiv import ArxivClient
from research_agent.config import OpenAIConfig
from research_agent.llm.openai_responses import OpenAIResearchLLM, OpenAIResponsesClient
from research_agent.orchestrator import ResearchOrchestrator, ResearchSettings


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run the local-first research agent.")
    subparsers = parser.add_subparsers(dest="command", required=True)

    run_parser = subparsers.add_parser("run", help="Run a research cycle.")
    run_parser.add_argument("--goal", required=True, help="Research goal to investigate.")
    run_parser.add_argument("--max-cycles", type=int, default=2)
    run_parser.add_argument("--max-papers", type=int, default=8)
    run_parser.add_argument("--max-ranked-papers", type=int, default=5)
    run_parser.add_argument("--max-repair-attempts", type=int, default=2)
    run_parser.add_argument("--execution-timeout", type=int, default=60)
    run_parser.add_argument("--runs-root", default="runs")
    run_parser.add_argument("--model", default=None)
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "run":
        config = OpenAIConfig.from_env(
            model_override=args.model,
            timeout_override=args.execution_timeout,
        )
        llm = OpenAIResearchLLM(OpenAIResponsesClient(config))
        orchestrator = ResearchOrchestrator(
            settings=ResearchSettings(
                max_cycles=args.max_cycles,
                max_papers=args.max_papers,
                max_ranked_papers=args.max_ranked_papers,
                max_repair_attempts=args.max_repair_attempts,
                execution_timeout_seconds=args.execution_timeout,
            ),
            search_client=ArxivClient(),
            llm=llm,
            runs_root=Path(args.runs_root),
        )
        report = orchestrator.run(args.goal)
        print(f"Run directory: {report.run_dir}")
        print(f"Final report: {report.final_report_path}")
        print(f"Completed cycles: {report.completed_cycles}")
        return 0

    parser.error(f"Unsupported command: {args.command}")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
