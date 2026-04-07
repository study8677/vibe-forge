from pathlib import Path
import shutil
import tempfile
import unittest
import sys

ROOT = Path(__file__).resolve().parents[1]
SRC = ROOT / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))

from video_ai_tool.runtime import detect_runtime


class RuntimeDetectionTests(unittest.TestCase):
    def test_detect_runtime_reports_gpu_when_nvidia_smi_exists(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            tmp = Path(tmpdir)
            ffmpeg = tmp / "ffmpeg"
            nvidia_smi = tmp / "nvidia-smi"
            ffmpeg.write_text("#!/bin/sh\nexit 0\n")
            nvidia_smi.write_text("#!/bin/sh\nexit 0\n")
            ffmpeg.chmod(0o755)
            nvidia_smi.chmod(0o755)

            original_which = shutil.which

            def fake_which(name: str) -> str | None:
                mapping = {"ffmpeg": str(ffmpeg), "nvidia-smi": str(nvidia_smi)}
                return mapping.get(name, original_which(name))

            try:
                shutil.which = fake_which  # type: ignore[assignment]
                runtime = detect_runtime(gpu_policy="required")
            finally:
                shutil.which = original_which  # type: ignore[assignment]

            self.assertTrue(runtime.ffmpeg_available)
            self.assertTrue(runtime.gpu_available)
            self.assertEqual(str(nvidia_smi), runtime.nvidia_smi_path)
