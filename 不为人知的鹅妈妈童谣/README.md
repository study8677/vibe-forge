# Video AI Tool

Local GPU-aware orchestration tool for a video post-processing chain:

- subtitle erasure
- TTS generation
- audio-driven lip sync
- AI face swap
- final merge into a single video

The repository does not bundle heavyweight model weights. Instead it provides a stable orchestration core that can call your own GPU-capable backends through command templates.

## What V1 does

- Loads a job from TOML
- Detects `ffmpeg` and optional NVIDIA GPU runtime
- Creates a per-job workspace under `.runs/`
- Executes configured stages in order
- Writes a manifest with commands, outputs, and statuses
- Supports `--dry-run` for safe validation

## Project layout

- `src/video_ai_tool/config.py`: TOML config parsing
- `src/video_ai_tool/runtime.py`: `ffmpeg` / GPU detection
- `src/video_ai_tool/adapters.py`: command template rendering
- `src/video_ai_tool/pipeline.py`: orchestration and manifest writing
- `src/video_ai_tool/cli.py`: command line interface
- `scripts/mock_backend.py`: fake backend for local verification
- `example_job.toml`: editable sample job

## Usage

Dry-run the sample job:

```bash
python3 -m video_ai_tool.cli run --job example_job.toml --dry-run
```

Run the sample job for real after you provide input assets:

```bash
python3 -m video_ai_tool.cli run --job example_job.toml
```

Force a rerun even if stage outputs already exist:

```bash
python3 -m video_ai_tool.cli run --job example_job.toml --force
```

## Input assets

Create an `input/` directory and provide:

- `input/source.mp4`
- `input/voice_reference.wav`
- `input/face_source.png`

Only `source.mp4` is used by the mock backend. The other assets are present so you can keep a stable job shape when replacing mock commands with real ones.

## Real backend wiring

Replace the stage commands in `example_job.toml` with your own GPU pipelines.

Typical patterns:

```toml
[stages.tts]
enabled = true
command = "CUDA_VISIBLE_DEVICES=0 python3 /opt/CosyVoice/infer.py --text-file {script_file} --prompt-wav {voice_reference} --out {output_audio}"
output = "tts.wav"
```

```toml
[stages.lip_sync]
enabled = true
command = "CUDA_VISIBLE_DEVICES=0 python3 /opt/MuseTalk/infer.py --video {input_video} --audio {input_audio} --output {output_video}"
output = "lip_synced.mp4"
```

```toml
[stages.face_swap]
enabled = true
command = "CUDA_VISIBLE_DEVICES=0 python3 /opt/FaceFusion/run.py headless-run --target {input_video} --source {face_source} --output {output_video}"
output = "face_swapped.mp4"
```

For subtitle erasure you can wire any OCR + inpainting or dedicated subtitle-removal pipeline:

```toml
[stages.subtitle_erase]
enabled = true
command = "CUDA_VISIBLE_DEVICES=0 python3 /opt/subtitle_eraser/run.py --input {input_video} --output {output_video}"
output = "subtitle_erased.mp4"
```

## GPU policy

`gpu_policy` accepts:

- `required`: fail if `nvidia-smi` is missing
- `preferred`: continue without GPU but report degraded mode
- `off`: skip GPU checks

## Manifest

Every run writes `.runs/<job-id>/manifest.json` with:

- planned stages
- commands
- output paths
- per-stage status
- captured stdout / stderr

## Tests

```bash
python3 -m unittest discover -s tests -v
```

## Notes

- This tool orchestrates local commands. It does not sandbox unsafe backend code.
- Face swap use must comply with your legal and consent requirements.
