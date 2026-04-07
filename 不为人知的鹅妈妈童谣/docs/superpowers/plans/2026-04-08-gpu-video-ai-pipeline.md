# GPU Video AI Pipeline Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a local Python CLI that validates a GPU-capable environment, orchestrates subtitle erasure, TTS, lip sync, face swap, and final composition through configurable external backends, and leaves resumable job artifacts on disk.

**Architecture:** Use a small Python package with a TOML job config, explicit stage modules, adapter-based shell command rendering, and a pipeline runner that records stage manifests. Keep the core dependency-light and move model-specific behavior into command templates.

**Tech Stack:** Python 3.11+, `argparse`, `dataclasses`, `pathlib`, `subprocess`, `tomllib`, `unittest`, `ffmpeg`

---

### Task 1: Project skeleton and config parsing

**Files:**
- Create: `pyproject.toml`
- Create: `src/video_ai_tool/__init__.py`
- Create: `src/video_ai_tool/config.py`
- Create: `src/video_ai_tool/models.py`
- Test: `tests/test_config.py`

- [ ] **Step 1: Write the failing test**

```python
def test_load_job_config_normalizes_relative_paths(tmp_path):
    ...
```

- [ ] **Step 2: Run test to verify it fails**

Run: `python -m unittest tests.test_config -v`
Expected: FAIL with `ModuleNotFoundError` for `video_ai_tool`

- [ ] **Step 3: Write minimal implementation**

Create package files and implement `load_job_config(path)` with TOML parsing plus relative-path normalization.

- [ ] **Step 4: Run test to verify it passes**

Run: `python -m unittest tests.test_config -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add pyproject.toml src/video_ai_tool/__init__.py src/video_ai_tool/config.py src/video_ai_tool/models.py tests/test_config.py
git commit -m "Create the job config foundation for the GPU video AI pipeline"
```

### Task 2: Runtime environment detection

**Files:**
- Create: `src/video_ai_tool/runtime.py`
- Test: `tests/test_runtime.py`

- [ ] **Step 1: Write the failing test**

```python
def test_detect_runtime_reports_gpu_when_nvidia_smi_exists():
    ...
```

- [ ] **Step 2: Run test to verify it fails**

Run: `python -m unittest tests.test_runtime -v`
Expected: FAIL because `detect_runtime` does not exist

- [ ] **Step 3: Write minimal implementation**

Implement binary checks for `ffmpeg` and `nvidia-smi`, plus GPU policy evaluation.

- [ ] **Step 4: Run test to verify it passes**

Run: `python -m unittest tests.test_runtime -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/video_ai_tool/runtime.py tests/test_runtime.py
git commit -m "Fail fast on missing ffmpeg or GPU runtime requirements"
```

### Task 3: Adapter command rendering

**Files:**
- Create: `src/video_ai_tool/adapters.py`
- Test: `tests/test_adapters.py`

- [ ] **Step 1: Write the failing test**

```python
def test_render_stage_command_substitutes_known_placeholders():
    ...
```

- [ ] **Step 2: Run test to verify it fails**

Run: `python -m unittest tests.test_adapters -v`
Expected: FAIL because adapter rendering is missing

- [ ] **Step 3: Write minimal implementation**

Implement a function that validates and renders configured command templates for each stage.

- [ ] **Step 4: Run test to verify it passes**

Run: `python -m unittest tests.test_adapters -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/video_ai_tool/adapters.py tests/test_adapters.py
git commit -m "Translate stage definitions into concrete backend commands"
```

### Task 4: Pipeline runner and manifest

**Files:**
- Create: `src/video_ai_tool/pipeline.py`
- Create: `src/video_ai_tool/manifest.py`
- Test: `tests/test_pipeline.py`

- [ ] **Step 1: Write the failing test**

```python
def test_pipeline_dry_run_plans_all_enabled_stages(tmp_path):
    ...
```

- [ ] **Step 2: Run test to verify it fails**

Run: `python -m unittest tests.test_pipeline -v`
Expected: FAIL because the pipeline runner does not exist

- [ ] **Step 3: Write minimal implementation**

Implement stage execution, dry-run planning, skip-if-output-exists behavior, and manifest writing.

- [ ] **Step 4: Run test to verify it passes**

Run: `python -m unittest tests.test_pipeline -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/video_ai_tool/pipeline.py src/video_ai_tool/manifest.py tests/test_pipeline.py
git commit -m "Record resumable stage execution for each video AI job"
```

### Task 5: CLI entry point

**Files:**
- Create: `src/video_ai_tool/cli.py`
- Test: `tests/test_cli.py`

- [ ] **Step 1: Write the failing test**

```python
def test_cli_dry_run_prints_stage_plan(tmp_path):
    ...
```

- [ ] **Step 2: Run test to verify it fails**

Run: `python -m unittest tests.test_cli -v`
Expected: FAIL because the CLI module is missing

- [ ] **Step 3: Write minimal implementation**

Add `python -m video_ai_tool.cli run --job <file> [--dry-run]` with readable output and non-zero exit on validation failure.

- [ ] **Step 4: Run test to verify it passes**

Run: `python -m unittest tests.test_cli -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/video_ai_tool/cli.py tests/test_cli.py
git commit -m "Expose the GPU video pipeline through a local CLI"
```

### Task 6: Documentation and sample job

**Files:**
- Create: `README.md`
- Create: `example_job.toml`

- [ ] **Step 1: Write the documentation**

Document setup, GPU policy, required binaries, sample backend commands, and dry-run usage.

- [ ] **Step 2: Verify sample job is valid**

Run: `python -m video_ai_tool.cli run --job example_job.toml --dry-run`
Expected: exit code `0` and a stage plan printed to stdout

- [ ] **Step 3: Commit**

```bash
git add README.md example_job.toml
git commit -m "Document how to wire external video AI backends into the pipeline"
```

## Self-review

Spec coverage:

- Orchestrator core: covered by Tasks 1, 4, 5
- GPU/runtime validation: covered by Task 2
- Adapter-based backend integration: covered by Task 3
- End-user documentation and sample config: covered by Task 6

Placeholder scan:

- No `TODO` / `TBD`
- Every task has explicit files and commands

Type consistency:

- `load_job_config`, `detect_runtime`, command rendering, and pipeline execution use a small shared model layer in `models.py`
