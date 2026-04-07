from __future__ import annotations

from pathlib import Path
import json

from .models import PipelineResult


def write_manifest(result: PipelineResult) -> Path:
    manifest_path = result.run_dir / "manifest.json"
    payload = {
        "run_dir": str(result.run_dir),
        "dry_run": result.dry_run,
        "planned_stages": result.planned_stages,
        "records": [
            {
                "name": record.name,
                "status": record.status,
                "command": record.command,
                "output_path": str(record.output_path),
                "skipped": record.skipped,
                "return_code": record.return_code,
                "stdout": record.stdout,
                "stderr": record.stderr,
            }
            for record in result.records
        ],
    }
    manifest_path.write_text(json.dumps(payload, indent=2, ensure_ascii=False))
    return manifest_path
