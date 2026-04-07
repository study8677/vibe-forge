from __future__ import annotations

from pathlib import Path
import tomllib

from .models import AssetsConfig, JobConfig, RuntimeConfig, StageConfig


STAGE_NAMES = ("subtitle_erase", "tts", "lip_sync", "face_swap")


def _resolve_optional_path(base_dir: Path, value: str | None) -> Path | None:
    if not value:
        return None
    path = Path(value)
    if not path.is_absolute():
        path = base_dir / path
    return path.resolve()


def load_job_config(path: str | Path) -> JobConfig:
    config_path = Path(path).resolve()
    config_dir = config_path.parent
    data = tomllib.loads(config_path.read_text())

    job = data.get("job", {})
    runtime = data.get("runtime", {})
    assets = data.get("assets", {})
    stages_data = data.get("stages", {})

    runtime_config = RuntimeConfig(
        work_root=_resolve_optional_path(config_dir, runtime.get("work_root")) or (config_dir / ".runs"),
        gpu_policy=runtime.get("gpu_policy", "preferred"),
    )
    assets_config = AssetsConfig(
        source_video=_resolve_optional_path(config_dir, assets["source_video"]),
        script_text=assets.get("script_text"),
        script_file=_resolve_optional_path(config_dir, assets.get("script_file")),
        voice_reference=_resolve_optional_path(config_dir, assets.get("voice_reference")),
        face_source=_resolve_optional_path(config_dir, assets.get("face_source")),
    )

    stages: dict[str, StageConfig] = {}
    for stage_name in STAGE_NAMES:
        stage = stages_data.get(stage_name, {})
        stages[stage_name] = StageConfig(
            name=stage_name,
            enabled=bool(stage.get("enabled", False)),
            command_template=stage.get("command", ""),
            output_name=stage.get("output", f"{stage_name}.out"),
        )

    return JobConfig(
        job_id=job.get("id", config_path.stem),
        config_path=config_path,
        config_dir=config_dir,
        assets=assets_config,
        runtime=runtime_config,
        stages=stages,
    )
