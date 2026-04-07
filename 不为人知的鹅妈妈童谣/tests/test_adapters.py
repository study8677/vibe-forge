import unittest
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from video_ai_tool.adapters import render_stage_command


class AdapterRenderTests(unittest.TestCase):
    def test_render_stage_command_substitutes_known_placeholders(self) -> None:
        command = render_stage_command(
            "python run.py --video {input_video} --out {output_video} --audio {input_audio}",
            {
                "input_video": "/tmp/input.mp4",
                "output_video": "/tmp/output.mp4",
                "input_audio": "/tmp/audio.wav",
            },
        )

        self.assertIn("/tmp/input.mp4", command)
        self.assertIn("/tmp/output.mp4", command)
        self.assertIn("/tmp/audio.wav", command)
