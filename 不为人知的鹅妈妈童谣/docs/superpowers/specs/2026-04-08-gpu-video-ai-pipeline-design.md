# GPU Video AI Pipeline Design

## Goal

Build a local GPU-aware tool that processes a source video through four stages:

1. hard-subtitle erasure
2. TTS generation from replacement script
3. audio-driven lip sync
4. AI face swap

The first deliverable is a runnable local pipeline orchestrator, not a turnkey packaged model bundle. The tool must be able to detect GPU availability, validate external tool configuration, execute each stage in order, preserve intermediate artifacts, and produce a final merged video.

## Scope

### In scope for V1

- Local command-line interface
- Declarative job config file
- GPU/runtime environment detection
- Stage-by-stage orchestration with resumable workspace outputs
- External backend adapters for:
  - subtitle erasure
  - TTS
  - lip sync
  - face swap
- Final audio/video composition through ffmpeg
- Dry-run mode for validating commands without running heavy inference
- Automated tests for core orchestration behavior

### Out of scope for V1

- Browser UI
- Distributed GPU scheduling
- Built-in download/installation of large model weights
- Fine-grained scene segmentation or speaker diarization
- Safety moderation beyond explicit project disclaimers

## Architecture

The system is a single Python application composed of focused modules:

- `cli`: parses the user command and job file
- `config`: validates the job definition and normalizes paths
- `runtime`: detects ffmpeg, `nvidia-smi`, CUDA hints, and working directories
- `pipeline`: coordinates stage execution and checkpoint files
- `stages`: one module per stage with a small, explicit interface
- `adapters`: translates a stage request into an external backend command
- `models`: small dataclasses describing job state, command results, and outputs

Each AI stage is executed via an adapter contract instead of hardcoding a single framework. This keeps the core system stable while allowing different backends such as MuseTalk, Wav2Lip, CosyVoice, or FaceFusion to be swapped by config.

## Data Flow

Input assets:

- source video
- target script text or subtitle file
- optional voice reference audio
- required face source image/video for face swap

Execution order:

1. validate files and runtime
2. create a job workspace under `.runs/<job-id>/`
3. export source metadata
4. erase hard subtitles from the source video
5. synthesize replacement speech
6. drive lip sync using the speech output
7. run face swap on the lip-synced video
8. merge final video + final audio
9. write a machine-readable manifest

Every stage writes its own output path and manifest entry. Re-running the same job can skip completed stages unless the user forces re-execution.

## Stage Contracts

### Subtitle Erasure

Input:

- source video
- optional subtitle region hint

Output:

- `subtitle_erased.mp4`

V1 does not implement the erasure model itself. Instead it runs a configured backend command and verifies the expected output path exists.

### TTS

Input:

- script text or subtitle text
- optional voice reference path
- language / speaker options

Output:

- `tts.wav`

The adapter must support placeholder substitution for input and output paths so model-specific commands can be configured without editing Python code.

### Lip Sync

Input:

- subtitle-erased video
- TTS audio

Output:

- `lip_synced.mp4`

### Face Swap

Input:

- lip-synced video
- face source asset

Output:

- `face_swapped.mp4`

### Compose

Input:

- final stage video
- TTS audio

Output:

- `final.mp4`

This stage is implemented directly with ffmpeg because the behavior is stable and deterministic.

## Configuration Format

The tool uses a TOML job file so it can be parsed from Python standard library `tomllib`.

The job file defines:

- job id
- paths for all assets
- runtime options
- adapter command templates
- optional per-stage enable/disable flags

Command templates may reference placeholders:

- `{input_video}`
- `{output_video}`
- `{input_audio}`
- `{output_audio}`
- `{script_file}`
- `{voice_reference}`
- `{face_source}`
- `{workdir}`

## Error Handling

- Missing required binaries fail fast with a clear message
- GPU policy:
  - `required`: fail if GPU unavailable
  - `preferred`: continue but mark degraded mode
  - `off`: skip GPU checks
- Each stage emits a status record with:
  - command
  - exit code
  - start/end timestamps
  - output path
- If a command exits zero but the expected artifact is missing, the stage fails
- The pipeline stops on the first failed stage

## Testing Strategy

Unit and integration-lite coverage for:

- TOML config parsing and path normalization
- GPU runtime detection
- stage planning and skip/resume behavior
- command template rendering
- dry-run output
- manifest generation

Heavy model inference is not part of automated tests. Tests use fake shell commands or Python one-liners to simulate stage outputs.

## Risks

- Real-world subtitle erasure quality depends entirely on the configured backend model
- Lip sync and face swap backends often have incompatible Python/CUDA stacks; adapters reduce coupling but do not remove environment complexity
- ffmpeg and GPU drivers may vary across machines; the tool should surface these issues clearly, not mask them

## Decision Summary

V1 is a local GPU-aware orchestration tool with real stage execution and resumable outputs. It deliberately avoids bundling heavyweight model implementations directly into the core codebase. That keeps the core small, testable, and adaptable while still delivering the requested workflow end to end.
