#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path
import shutil
import wave


def write_silence_wav(path: Path, seconds: float = 1.0, sample_rate: int = 16000) -> None:
    frame_count = int(seconds * sample_rate)
    path.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(path), "wb") as wav:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(sample_rate)
        wav.writeframes(b"\x00\x00" * frame_count)


def main() -> int:
    parser = argparse.ArgumentParser(description="Mock backend for pipeline verification")
    parser.add_argument("--mode", choices=["video-copy", "audio-silence"], required=True)
    parser.add_argument("--input", help="Input file for copy mode")
    parser.add_argument("--output", required=True, help="Output file path")
    args = parser.parse_args()

    output = Path(args.output).resolve()
    output.parent.mkdir(parents=True, exist_ok=True)

    if args.mode == "video-copy":
        if not args.input:
            raise SystemExit("--input is required for video-copy")
        shutil.copyfile(args.input, output)
        return 0

    write_silence_wav(output)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
