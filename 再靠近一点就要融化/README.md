# Research Agent

A local-first Python research agent that turns a research goal into a repeatable loop:

1. Search arXiv
2. Rank and read papers
3. Generate an experiment idea
4. Generate runnable Python code
5. Execute and repair the code until it passes or the retry budget is exhausted
6. Evaluate the result and optionally continue into another cycle

## Quickstart

```bash
export OPENAI_API_KEY=your_api_key
python -m research_agent.cli run --goal "Design a lightweight multimodal retrieval benchmark"
```

Artifacts are written to `runs/<timestamp>-<goal-slug>/`.

## Notes

- The first version executes Python only.
- Generated code is stored before execution.
- The repair loop only edits generated Python scripts. It does not install packages.
