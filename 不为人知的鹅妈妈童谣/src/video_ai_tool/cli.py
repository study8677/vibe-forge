from __future__ import annotations

import argparse
import sys

from .config import load_job_config
from .pipeline import PipelineRunner
from .runtime import detect_runtime


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="GPU-aware local video AI pipeline")
    subparsers = parser.add_subparsers(dest="command", required=True)

    run_parser = subparsers.add_parser("run", help="Run a job from a TOML file")
    run_parser.add_argument("--job", required=True, help="Path to the TOML job file")
    run_parser.add_argument("--dry-run", action="store_true", help="Plan commands without running them")
    run_parser.add_argument("--force", action="store_true", help="Re-run stages even if outputs already exist")
    run_parser.add_argument("--skip-runtime-check", action="store_true", help="Skip ffmpeg/GPU detection")
    return parser


def main(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    if args.command != "run":
        return 1

    config = load_job_config(args.job)
    if not args.skip_runtime_check and not args.dry_run:
        detect_runtime(config.runtime.gpu_policy)

    result = PipelineRunner(config).run(dry_run=args.dry_run, force=args.force)
    for stage in result.records:
        print(f"{stage.name}: {stage.status}")
        print(f"  output: {stage.output_path}")
        print(f"  command: {stage.command}")
    print(f"manifest: {result.run_dir / 'manifest.json'}")
    return 0 if all(record.status != "failed" for record in result.records) else 1


if __name__ == "__main__":
    sys.exit(main())
