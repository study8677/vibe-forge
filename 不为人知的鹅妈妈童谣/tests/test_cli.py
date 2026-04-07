from pathlib import Path
import io
import tempfile
import textwrap
import unittest
import sys
from contextlib import redirect_stdout

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from video_ai_tool.cli import main


class CliTests(unittest.TestCase):
    def test_cli_dry_run_prints_stage_plan(self) -> None:
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

            output = io.StringIO()
            with redirect_stdout(output):
                code = main(["run", "--job", str(job), "--dry-run"])

            self.assertEqual(0, code)
            self.assertIn("subtitle_erase", output.getvalue())
            self.assertIn("compose", output.getvalue())
