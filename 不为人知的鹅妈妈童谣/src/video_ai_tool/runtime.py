from __future__ import annotations

import shutil

from .models import RuntimeInfo


def detect_runtime(gpu_policy: str = "preferred") -> RuntimeInfo:
    ffmpeg_path = shutil.which("ffmpeg")
    ffmpeg_available = ffmpeg_path is not None
    if not ffmpeg_available:
        raise RuntimeError("ffmpeg is required but was not found in PATH")

    nvidia_smi_path = shutil.which("nvidia-smi")
    gpu_available = nvidia_smi_path is not None
    warnings: list[str] = []

    if gpu_policy == "required" and not gpu_available:
        raise RuntimeError("GPU policy is 'required' but nvidia-smi was not found in PATH")
    if gpu_policy == "preferred" and not gpu_available:
        warnings.append("GPU not detected; continuing in degraded mode")

    return RuntimeInfo(
        ffmpeg_available=ffmpeg_available,
        ffmpeg_path=ffmpeg_path,
        gpu_available=gpu_available,
        nvidia_smi_path=nvidia_smi_path,
        gpu_policy=gpu_policy,
        warnings=warnings,
    )
