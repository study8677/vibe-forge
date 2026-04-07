from pathlib import Path
import tempfile
import textwrap
import unittest
import sys

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from video_ai_tool.config import load_job_config


class LoadJobConfigTests(unittest.TestCase):
    def test_load_job_config_normalizes_relative_paths(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir)
            (tmp / "assets").mkdir()
            (tmp / "assets" / "input.mp4").write_text("video")
            (tmp / "assets" / "voice.wav").write_text("audio")
            (tmp / "assets" / "face.png").write_text("face")
            job_file = tmp / "job.toml"
            job_file.write_text(
                textwrap.dedent(
                    """
                    [job]
                    id = "demo"

                    [runtime]
                    work_root = ".runs"
                    gpu_policy = "preferred"

                    [assets]
                    source_video = "assets/input.mp4"
                    voice_reference = "assets/voice.wav"
                    face_source = "assets/face.png"
                    script_text = "hello world"

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

            config = load_job_config(job_file)

            self.assertEqual("demo", config.job_id)
            self.assertEqual((tmp / "assets" / "input.mp4").resolve(), config.assets.source_video)
            self.assertEqual((tmp / ".runs").resolve(), config.runtime.work_root)
            self.assertEqual("subtitle_erased.mp4", config.stages["subtitle_erase"].output_name)
