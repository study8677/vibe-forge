# Zed Lines History

Prototype implementation of a recursive Git line-history engine for a future Zed "Lines History" experience.

## Why this repository exists

Zed's current public extension API does not expose the native panel/editor-event hooks needed to implement a GitLens-style line-history pane entirely as a marketplace extension. This repository therefore focuses on:

- a reusable Rust core that resolves line ancestry across revisions
- a thin CLI prototype that demonstrates the behavior today
- an integration boundary suitable for a future Zed core/fork implementation

## Planned CLI usage

```bash
cargo run -p zed-lines-history-cli -- \
  --repo /path/to/repo \
  --rev HEAD \
  --path src/lib.rs \
  --line 42 \
  --format text
```

## Recursive traversal workflow

The CLI returns one history edge at a time:

- current revision snapshot
- previous revision snapshot
- mapping confidence

To continue traversing backward, take the `previous` snapshot's `commit`, `path`, and `line`, then call the CLI again with those values.

Example:

```bash
cargo run -p zed-lines-history-cli -- \
  --repo /path/to/repo \
  --rev HEAD~1 \
  --path src/lib.rs \
  --line 37 \
  --format json
```

This is the same traversal model that a future native Zed `Lines History` panel would use internally.
