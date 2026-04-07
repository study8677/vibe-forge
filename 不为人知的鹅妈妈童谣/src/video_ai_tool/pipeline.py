from __future__ import annotations

from pathlib import Path
import subprocess

from .adapters import render_stage_command
from .manifest import write_manifest
from .models import JobConfig, PipelineResult, StageRecord


class PipelineRunner:
    def __init__(self, config: JobConfig) -> None:
        self.config = config

    def run(self, dry_run: bool = False, force: bool = False) -> PipelineResult:
        run_dir = self.config.runtime.work_root / self.config.job_id
        run_dir.mkdir(parents=True, exist_ok=True)
        script_file = self._ensure_script_file(run_dir)
        planned_stages = [name for name, stage in self.config.stages.items() if stage.enabled]
        planned_stages.append("compose")

        outputs: dict[str, Path] = {"source_video": self.config.assets.source_video}
        if self.config.assets.voice_reference:
            outputs["voice_reference"] = self.config.assets.voice_reference
        if self.config.assets.face_source:
            outputs["face_source"] = self.config.assets.face_source
        if script_file:
            outputs["script_file"] = script_file

        records: list[StageRecord] = []
        for stage_name in planned_stages:
            if stage_name == "compose":
                record = self._run_compose(run_dir, outputs, dry_run=dry_run, force=force)
            else:
                record = self._run_stage(stage_name, run_dir, outputs, dry_run=dry_run, force=force)
            records.append(record)
            outputs[stage_name] = record.output_path
            if record.status == "failed":
                break

        result = PipelineResult(run_dir=run_dir, planned_stages=planned_stages, records=records, dry_run=dry_run)
        write_manifest(result)
        return result

    def _ensure_script_file(self, run_dir: Path) -> Path | None:
        if self.config.assets.script_file:
            return self.config.assets.script_file
        if self.config.assets.script_text:
            script_path = run_dir / "script.txt"
            script_path.write_text(self.config.assets.script_text)
            return script_path
        return None

    def _base_placeholders(self, run_dir: Path, output_path: Path, outputs: dict[str, Path]) -> dict[str, str]:
        placeholders = {
            "workdir": str(run_dir),
            "input_video": str(outputs.get("face_swap") or outputs.get("lip_sync") or outputs.get("subtitle_erase") or self.config.assets.source_video),
            "output_video": str(output_path),
            "input_audio": str(outputs.get("tts", "")),
            "output_audio": str(output_path),
            "script_file": str(outputs.get("script_file", "")),
            "voice_reference": str(self.config.assets.voice_reference or ""),
            "face_source": str(self.config.assets.face_source or ""),
        }
        return placeholders

    def _run_stage(
        self,
        stage_name: str,
        run_dir: Path,
        outputs: dict[str, Path],
        *,
        dry_run: bool,
        force: bool,
    ) -> StageRecord:
        stage = self.config.stages[stage_name]
        output_path = run_dir / stage.output_name
        placeholders = self._base_placeholders(run_dir, output_path, outputs)

        if stage_name == "subtitle_erase":
            placeholders["input_video"] = str(self.config.assets.source_video)
        elif stage_name == "tts":
            placeholders["output_audio"] = str(output_path)
        elif stage_name == "lip_sync":
            placeholders["input_video"] = str(outputs["subtitle_erase"])
            placeholders["input_audio"] = str(outputs["tts"])
            placeholders["output_video"] = str(output_path)
        elif stage_name == "face_swap":
            placeholders["input_video"] = str(outputs["lip_sync"])
            placeholders["output_video"] = str(output_path)

        command = render_stage_command(stage.command_template, placeholders)

        if output_path.exists() and not force:
            return StageRecord(
                name=stage_name,
                status="skipped",
                command=command,
                output_path=output_path,
                skipped=True,
            )
        if dry_run:
            return StageRecord(name=stage_name, status="planned", command=command, output_path=output_path)

        completed = subprocess.run(
            command,
            shell=True,
            cwd=self.config.config_dir,
            text=True,
            capture_output=True,
            check=False,
        )
        status = "completed"
        if completed.returncode != 0:
            status = "failed"
        elif not output_path.exists():
            status = "failed"
            completed.stderr = f"{completed.stderr}\nExpected output missing: {output_path}".strip()

        return StageRecord(
            name=stage_name,
            status=status,
            command=command,
            output_path=output_path,
            stdout=completed.stdout,
            stderr=completed.stderr,
            return_code=completed.returncode,
        )

    def _run_compose(self, run_dir: Path, outputs: dict[str, Path], *, dry_run: bool, force: bool) -> StageRecord:
        output_path = run_dir / "final.mp4"
        input_video = outputs.get("face_swap") or outputs.get("lip_sync") or outputs.get("subtitle_erase") or self.config.assets.source_video
        input_audio = outputs.get("tts")
        command = (
            f'ffmpeg -y -i "{input_video}" -i "{input_audio}" '
            f'-map 0:v:0 -map 1:a:0 -c:v copy -shortest "{output_path}"'
        )

        if output_path.exists() and not force:
            return StageRecord(name="compose", status="skipped", command=command, output_path=output_path, skipped=True)
        if dry_run:
            return StageRecord(name="compose", status="planned", command=command, output_path=output_path)

        completed = subprocess.run(
            command,
            shell=True,
            cwd=self.config.config_dir,
            text=True,
            capture_output=True,
            check=False,
        )
        status = "completed" if completed.returncode == 0 and output_path.exists() else "failed"
        if status == "failed" and completed.returncode == 0:
            completed.stderr = f"{completed.stderr}\nExpected output missing: {output_path}".strip()

        return StageRecord(
            name="compose",
            status=status,
            command=command,
            output_path=output_path,
            stdout=completed.stdout,
            stderr=completed.stderr,
            return_code=completed.returncode,
        )
