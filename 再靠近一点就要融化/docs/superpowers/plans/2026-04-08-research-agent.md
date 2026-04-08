# Research Agent Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Python CLI research agent that loops through arXiv retrieval, paper ranking and reading, idea generation, code generation, execution repair, evaluation, and multi-cycle iteration.

**Architecture:** Use a local-first orchestrator with file-based run artifacts and a high-level LLM protocol. Keep each module narrowly scoped so the orchestration loop can be tested with fake search and model backends.

**Tech Stack:** Python 3.14, stdlib HTTP/XML/subprocess/dataclasses, OpenAI Responses API over raw HTTPS, `unittest`, `ruff`

---

### Task 1: Project Skeleton

**Files:**
- Create: `pyproject.toml`
- Create: `src/research_agent/__init__.py`
- Create: `README.md`

- [ ] **Step 1: Write the failing test**

```python
import importlib
import unittest


class PackageImportTest(unittest.TestCase):
    def test_package_imports(self) -> None:
        module = importlib.import_module("research_agent")
        self.assertEqual(module.__all__, ["__version__"])
```

- [ ] **Step 2: Run test to verify it fails**

Run: `python3 -m unittest tests.test_package_import -v`
Expected: FAIL with `ModuleNotFoundError: No module named 'research_agent'`

- [ ] **Step 3: Write minimal implementation**

```python
__version__ = "0.1.0"
__all__ = ["__version__"]
```

- [ ] **Step 4: Run test to verify it passes**

Run: `PYTHONPATH=src python3 -m unittest tests.test_package_import -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add pyproject.toml src/research_agent/__init__.py README.md tests/test_package_import.py
git commit -m "Create the research agent package scaffold"
```

### Task 2: Typed Domain Models

**Files:**
- Create: `src/research_agent/models.py`
- Test: `tests/test_models.py`

- [ ] **Step 1: Write the failing test**

```python
import unittest

from research_agent.models import ExecutionResult, ExperimentIdea, Paper


class ModelShapeTest(unittest.TestCase):
    def test_models_capture_core_fields(self) -> None:
        paper = Paper(
            paper_id="1234.5678",
            title="A Paper",
            summary="Summary",
            authors=["Alice"],
            pdf_url="https://arxiv.org/pdf/1234.5678.pdf",
            published="2026-01-01T00:00:00Z",
        )
        idea = ExperimentIdea(
            title="Try a baseline",
            hypothesis="The baseline will converge",
            method="Train a small model",
            success_metric="validation accuracy",
        )
        result = ExecutionResult(success=True, returncode=0, stdout="ok", stderr="")
        self.assertEqual(paper.paper_id, "1234.5678")
        self.assertEqual(idea.success_metric, "validation accuracy")
        self.assertTrue(result.success)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PYTHONPATH=src python3 -m unittest tests.test_models -v`
Expected: FAIL with `ModuleNotFoundError: No module named 'research_agent.models'`

- [ ] **Step 3: Write minimal implementation**

```python
from dataclasses import dataclass, field


@dataclass(slots=True)
class Paper:
    paper_id: str
    title: str
    summary: str
    authors: list[str]
    pdf_url: str
    published: str


@dataclass(slots=True)
class ExperimentIdea:
    title: str
    hypothesis: str
    method: str
    success_metric: str
    risks: list[str] = field(default_factory=list)


@dataclass(slots=True)
class ExecutionResult:
    success: bool
    returncode: int
    stdout: str
    stderr: str
```

- [ ] **Step 4: Run test to verify it passes**

Run: `PYTHONPATH=src python3 -m unittest tests.test_models -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/research_agent/models.py tests/test_models.py
git commit -m "Define typed models for the research workflow"
```

### Task 3: arXiv Search and Parsing

**Files:**
- Create: `src/research_agent/arxiv.py`
- Test: `tests/test_arxiv.py`

- [ ] **Step 1: Write the failing test**

```python
import unittest

from research_agent.arxiv import parse_arxiv_feed


SAMPLE_FEED = """<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <id>http://arxiv.org/abs/1234.5678v1</id>
    <updated>2026-04-01T00:00:00Z</updated>
    <published>2026-04-01T00:00:00Z</published>
    <title> Sample Paper </title>
    <summary> Example abstract. </summary>
    <author><name>Alice</name></author>
    <author><name>Bob</name></author>
    <link title="pdf" href="http://arxiv.org/pdf/1234.5678v1"/>
  </entry>
</feed>
"""


class ArxivParserTest(unittest.TestCase):
    def test_parse_arxiv_feed_extracts_papers(self) -> None:
        papers = parse_arxiv_feed(SAMPLE_FEED)
        self.assertEqual(len(papers), 1)
        self.assertEqual(papers[0].paper_id, "1234.5678v1")
        self.assertEqual(papers[0].authors, ["Alice", "Bob"])
        self.assertEqual(papers[0].title, "Sample Paper")
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PYTHONPATH=src python3 -m unittest tests.test_arxiv -v`
Expected: FAIL with `cannot import name 'parse_arxiv_feed'`

- [ ] **Step 3: Write minimal implementation**

```python
import xml.etree.ElementTree as ET

from research_agent.models import Paper

ATOM_NS = {"atom": "http://www.w3.org/2005/Atom"}


def parse_arxiv_feed(xml_text: str) -> list[Paper]:
    root = ET.fromstring(xml_text)
    papers: list[Paper] = []
    for entry in root.findall("atom:entry", ATOM_NS):
        paper_id = entry.findtext("atom:id", default="", namespaces=ATOM_NS).rsplit("/", 1)[-1]
        title = " ".join(entry.findtext("atom:title", default="", namespaces=ATOM_NS).split())
        summary = " ".join(entry.findtext("atom:summary", default="", namespaces=ATOM_NS).split())
        authors = [author.findtext("atom:name", default="", namespaces=ATOM_NS) for author in entry.findall("atom:author", ATOM_NS)]
        pdf_url = ""
        for link in entry.findall("atom:link", ATOM_NS):
            if link.attrib.get("title") == "pdf":
                pdf_url = link.attrib.get("href", "")
                break
        papers.append(
            Paper(
                paper_id=paper_id,
                title=title,
                summary=summary,
                authors=authors,
                pdf_url=pdf_url,
                published=entry.findtext("atom:published", default="", namespaces=ATOM_NS),
            )
        )
    return papers
```

- [ ] **Step 4: Run test to verify it passes**

Run: `PYTHONPATH=src python3 -m unittest tests.test_arxiv -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/research_agent/arxiv.py tests/test_arxiv.py
git commit -m "Add arXiv feed parsing for paper retrieval"
```

### Task 4: Research Loop Repair Behavior

**Files:**
- Create: `src/research_agent/workspace.py`
- Create: `src/research_agent/executor.py`
- Create: `src/research_agent/orchestrator.py`
- Create: `src/research_agent/llm/base.py`
- Test: `tests/test_research_loop.py`

- [ ] **Step 1: Write the failing test**

```python
import tempfile
import unittest
from pathlib import Path

from research_agent.models import EvaluationResult, ExperimentIdea, Paper, PaperDigest, RankedPaper
from research_agent.orchestrator import ResearchOrchestrator, ResearchSettings


class FakeSearch:
    def search(self, goal: str, limit: int) -> list[Paper]:
        return [
            Paper(
                paper_id="1234.5678v1",
                title="Sample Paper",
                summary="Abstract",
                authors=["Alice"],
                pdf_url="http://arxiv.org/pdf/1234.5678v1",
                published="2026-04-01T00:00:00Z",
            )
        ]


class FakeLLM:
    def __init__(self) -> None:
        self.repair_calls = 0

    def rank_papers(self, goal: str, papers: list[Paper]) -> list[RankedPaper]:
        return [RankedPaper(paper=papers[0], relevance_score=0.95, rationale="Exact topic match")]

    def digest_papers(self, goal: str, ranked_papers: list[RankedPaper]) -> list[PaperDigest]:
        return [PaperDigest(paper_id=ranked_papers[0].paper.paper_id, takeaway="Useful baseline", relevance="High")]

    def generate_idea(self, goal: str, digests: list[PaperDigest], previous_cycles: list[object]) -> ExperimentIdea:
        return ExperimentIdea(
            title="Baseline experiment",
            hypothesis="A baseline script should run",
            method="Print a success marker",
            success_metric="script exits successfully",
        )

    def generate_code(self, goal: str, idea: ExperimentIdea, digests: list[PaperDigest], previous_attempts: list[object]) -> str:
        return "raise RuntimeError('boom')\n"

    def repair_code(self, goal: str, idea: ExperimentIdea, broken_code: str, error_context: str, previous_attempts: list[object]) -> str:
        self.repair_calls += 1
        return "print('success')\n"

    def evaluate_cycle(self, goal: str, idea: ExperimentIdea, execution_result: object, digests: list[PaperDigest], previous_cycles: list[object]) -> EvaluationResult:
        return EvaluationResult(
            summary="Execution succeeded",
            strengths=["Recovered from failure"],
            limitations=[],
            confidence=0.8,
            recommendation="stop",
            next_step="",
        )


class ResearchLoopTest(unittest.TestCase):
    def test_research_loop_repairs_code_until_success(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            orchestrator = ResearchOrchestrator(
                settings=ResearchSettings(max_cycles=1, max_papers=3, max_repair_attempts=2),
                search_client=FakeSearch(),
                llm=FakeLLM(),
                runs_root=Path(tmpdir),
            )
            report = orchestrator.run("test repair loop")
            self.assertEqual(report.completed_cycles, 1)
            self.assertTrue(report.cycles[0].execution.success)
            self.assertEqual(report.cycles[0].repair_attempts, 1)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PYTHONPATH=src python3 -m unittest tests.test_research_loop -v`
Expected: FAIL with missing orchestrator and model types

- [ ] **Step 3: Write minimal implementation**

```python
class ResearchSettings:
    ...


class ResearchOrchestrator:
    def run(self, goal: str) -> SessionReport:
        ...
```

Implement:

- a run directory creator
- a Python subprocess executor
- one-cycle loop with repair retries
- a session report containing cycle results

- [ ] **Step 4: Run test to verify it passes**

Run: `PYTHONPATH=src python3 -m unittest tests.test_research_loop -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/research_agent/workspace.py src/research_agent/executor.py src/research_agent/orchestrator.py src/research_agent/llm/base.py tests/test_research_loop.py
git commit -m "Implement the repair-aware research loop"
```

### Task 5: Multi-Cycle Evaluation Control

**Files:**
- Modify: `src/research_agent/models.py`
- Modify: `src/research_agent/orchestrator.py`
- Test: `tests/test_multi_cycle.py`

- [ ] **Step 1: Write the failing test**

```python
import tempfile
import unittest
from pathlib import Path

from research_agent.orchestrator import ResearchOrchestrator, ResearchSettings
from tests.test_research_loop import FakeLLM, FakeSearch


class MultiCycleLLM(FakeLLM):
    def __init__(self) -> None:
        super().__init__()
        self.eval_calls = 0

    def evaluate_cycle(self, goal, idea, execution_result, digests, previous_cycles):
        self.eval_calls += 1
        recommendation = "continue" if self.eval_calls == 1 else "stop"
        return EvaluationResult(
            summary="Cycle complete",
            strengths=["Produced an output"],
            limitations=[],
            confidence=0.7,
            recommendation=recommendation,
            next_step="Try a refined variant" if recommendation == "continue" else "",
        )


class MultiCycleTest(unittest.TestCase):
    def test_orchestrator_runs_multiple_cycles_when_requested(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            orchestrator = ResearchOrchestrator(
                settings=ResearchSettings(max_cycles=2, max_papers=3, max_repair_attempts=1),
                search_client=FakeSearch(),
                llm=MultiCycleLLM(),
                runs_root=Path(tmpdir),
            )
            report = orchestrator.run("test multi cycle")
            self.assertEqual(report.completed_cycles, 2)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PYTHONPATH=src python3 -m unittest tests.test_multi_cycle -v`
Expected: FAIL because the orchestrator stops after the first successful cycle

- [ ] **Step 3: Write minimal implementation**

```python
while cycle_index < settings.max_cycles:
    ...
    if evaluation.recommendation == "stop":
        break
```

Add explicit cycle state propagation so the next cycle sees prior evaluations.

- [ ] **Step 4: Run test to verify it passes**

Run: `PYTHONPATH=src python3 -m unittest tests.test_multi_cycle -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/research_agent/models.py src/research_agent/orchestrator.py tests/test_multi_cycle.py
git commit -m "Add evaluation-driven multi-cycle research iteration"
```

### Task 6: OpenAI Backend and CLI

**Files:**
- Create: `src/research_agent/config.py`
- Create: `src/research_agent/llm/openai_responses.py`
- Create: `src/research_agent/cli.py`
- Modify: `README.md`
- Test: `tests/test_cli.py`

- [ ] **Step 1: Write the failing test**

```python
import tempfile
import unittest
from pathlib import Path

from research_agent.cli import build_parser


class CliParserTest(unittest.TestCase):
    def test_cli_parses_goal_and_cycle_flags(self) -> None:
        parser = build_parser()
        args = parser.parse_args(["run", "--goal", "study agents", "--max-cycles", "2"])
        self.assertEqual(args.command, "run")
        self.assertEqual(args.goal, "study agents")
        self.assertEqual(args.max_cycles, 2)
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PYTHONPATH=src python3 -m unittest tests.test_cli -v`
Expected: FAIL with missing CLI module

- [ ] **Step 3: Write minimal implementation**

```python
import argparse


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("--goal", required=True)
    run_parser.add_argument("--max-cycles", type=int, default=2)
    return parser
```

Then extend it to:

- read API settings from environment
- instantiate the OpenAI-backed LLM
- run the orchestrator
- print the final report path

- [ ] **Step 4: Run test to verify it passes**

Run: `PYTHONPATH=src python3 -m unittest tests.test_cli -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/research_agent/config.py src/research_agent/llm/openai_responses.py src/research_agent/cli.py README.md tests/test_cli.py
git commit -m "Wire the OpenAI backend into the research agent CLI"
```

### Task 7: Verification Sweep

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Run unit tests**

Run: `PYTHONPATH=src python3 -m unittest discover -s tests -v`
Expected: PASS

- [ ] **Step 2: Run lint**

Run: `ruff check .`
Expected: `All checks passed!`

- [ ] **Step 3: Run static compilation**

Run: `PYTHONPATH=src python3 -m compileall src tests`
Expected: no syntax errors

- [ ] **Step 4: Update docs if command examples drifted**

```markdown
## Quickstart

export OPENAI_API_KEY=...
python -m research_agent.cli run --goal "..."
```

- [ ] **Step 5: Commit**

```bash
git add README.md
git commit -m "Document and verify the research agent workflow"
```
