# Research Agent Design

## Goal

Build a local-first research agent that takes a research goal and runs a repeatable loop:

1. Query arXiv for candidate papers.
2. Rank and read the most relevant papers.
3. Generate experiment ideas.
4. Generate executable research code.
5. Run the code locally and repair failures until it passes or the retry budget is exhausted.
6. Evaluate the result and decide whether to stop or begin another research cycle.

The first version is a Python CLI application with file-based session artifacts. It avoids web UI, background services, and third-party orchestration frameworks.

## Why This Shape

This repository is empty, so the fastest path to a useful system is a composable CLI with a small set of focused modules. The core risks in this problem are orchestration stability, local execution safety, and reproducibility. A local CLI with explicit state files is easier to test and extend than a service-first design.

## User Interface

The primary entrypoint is a command like:

```bash
python -m research_agent.cli run --goal "Design a lightweight multimodal retrieval benchmark"
```

Expected behavior:

- Accept a research goal and optional loop parameters.
- Create a timestamped run directory under `runs/`.
- Persist papers, rankings, summaries, ideas, code attempts, execution logs, and evaluations.
- Print a concise progress summary during execution.
- Emit a final Markdown report and a machine-readable JSON summary.

## Architecture

The system is split into clear single-purpose modules:

- `config.py`: environment and CLI settings.
- `models.py`: typed dataclasses for papers, ideas, code drafts, execution results, and evaluations.
- `arxiv.py`: arXiv query and Atom feed parsing.
- `workspace.py`: run directory creation and artifact persistence.
- `executor.py`: isolated local Python execution and output capture.
- `orchestrator.py`: end-to-end research loop.
- `llm/base.py`: protocol for high-level research actions.
- `llm/openai_responses.py`: OpenAI Responses API implementation of the protocol.
- `cli.py`: command-line entrypoint and summary output.

## Data Flow

For each research cycle:

1. Search arXiv with the goal string.
2. Ask the LLM to rank the papers and explain relevance.
3. Ask the LLM to summarize the top papers into structured reading notes.
4. Ask the LLM for one experiment idea grounded in those notes and prior cycle feedback.
5. Ask the LLM to generate a runnable Python script.
6. Execute the script locally.
7. If execution fails, ask the LLM to repair the script using the stderr and prior attempt.
8. Once execution succeeds or retries are exhausted, ask the LLM to evaluate the result.
9. If evaluation recommends continuing and cycle budget remains, repeat with evaluation feedback included in the next cycle.

## Execution Model

The agent writes generated code into a per-cycle working directory and executes it with `python3` in a subprocess. Each attempt has its own source file and execution log. The first version is intentionally simple:

- Python-only execution.
- No shell tool use from model output.
- No package installation loop.
- No Docker sandbox.

The repair loop focuses on fixing Python runtime and syntax failures inside the generated script.

## Safety Constraints

- Generated code is stored before execution for inspection and reproducibility.
- Execution is local and limited to Python files created in the run directory.
- The orchestrator has explicit caps for paper count, cycle count, and repair attempts.
- The CLI fails fast if `OPENAI_API_KEY` is missing when the OpenAI backend is selected.

## Evaluation Strategy

Each cycle produces a structured evaluation containing:

- outcome summary
- strengths
- limitations
- confidence score
- recommendation: `stop` or `continue`
- next-cycle guidance

The final report aggregates all cycles and points to the best successful attempt.

## Testing Strategy

The first version is tested without real network or model calls by injecting fake implementations:

- arXiv feed parsing test
- repair loop test: failing code is repaired and rerun
- multi-cycle test: evaluator asks for another cycle before stopping

This gives high confidence in orchestration behavior even when live APIs are unavailable.

## Deferred Work

The first version intentionally excludes:

- PDF download and full paper parsing
- citation graph expansion
- automatic package installation
- notebook generation
- dataset management
- parallel experiment branches
- UI/dashboard

These can be added later without breaking the core module boundaries.
