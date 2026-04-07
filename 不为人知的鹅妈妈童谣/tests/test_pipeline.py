from pathlib import Path
import json
import tempfile
import textwrap
import unittest
import sys

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from video_ai_tool.config import load_job_config
from video_ai_tool.pipeline import PipelineRunner


class PipelineRunnerTests(unittest.TestCase):
    def test_pipeline_dry_run_plans_all_enabled_stages(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = Path(tmpdir)
            (root / "assets").mkdir()
            (root / "assets" / "input.mp4").write_text("video")
            (root / "assets" / "voice.wav").write_text("audio")
            (root / "assets" / "face.png").write_text("face")
            job = root / "job.toml"
            job.write_text(
                textwrap.dedent(
                    """
                    [job]
                    id = "demo"

                    [runtime]
                    work_root = ".runs"
                    gpu_policy = "off"

                    [assets]
                    source_video = "assets/input.mp4"
                    voice_reference = "assets/voice.wav"
                    face_source = "assets/face.png"
                    script_text = "hello"

                    [stages.subtitle_erase]
                    enabled = true
                    command = "echo erase"
                    output = "subtitle_erased.mp4"

                    [stages.tts]
                    enabled = true
                    command = "echo tts"
                    output = "tts.wav"

                    [stages.lip_sync]
                    enabled = true
                    command = "echo lips"
                    output = "lip_synced.mp4"

                    [stages.face_swap]
                    enabled = true
                    command = "echo swap"
                    output = "face_swapped.mp4"
                    """
                ).strip()
            )

            config = load_job_config(job)
            result = PipelineRunner(config).run(dry_run=True)

            self.assertEqual(["subtitle_erase", "tts", "lip_sync", "face_swap", "compose"], result.planned_stages)
            manifest = json.loads((result.run_dir / "manifest.json").read_text())
            self.assertTrue(manifest["dry_run"])
