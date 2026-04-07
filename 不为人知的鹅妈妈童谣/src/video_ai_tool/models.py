from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path


@dataclass(slots=True)
class AssetsConfig:
    source_video: Path
    script_text: str | None = None
    script_file: Path | None = None
    voice_reference: Path | None = None
    face_source: Path | None = None


@dataclass(slots=True)
class RuntimeConfig:
    work_root: Path
    gpu_policy: str = "preferred"


@dataclass(slots=True)
class StageConfig:
    name: str
    enabled: bool
    command_template: str
    output_name: str


@dataclass(slots=True)
class JobConfig:
    job_id: str
    config_path: Path
    config_dir: Path
    assets: AssetsConfig
    runtime: RuntimeConfig
    stages: dict[str, StageConfig]


@dataclass(slots=True)
class RuntimeInfo:
    ffmpeg_available: bool
    ffmpeg_path: str | None
    gpu_available: bool
    nvidia_smi_path: str | None
    gpu_policy: str
    warnings: list[str] = field(default_factory=list)


@dataclass(slots=True)
class StageRecord:
    name: str
    status: str
    command: str
    output_path: Path
    skipped: bool = False
    stdout: str = ""
    stderr: str = ""
    return_code: int | None = None


@dataclass(slots=True)
class PipelineResult:
    run_dir: Path
    planned_stages: list[str]
    records: list[StageRecord]
    dry_run: bool
