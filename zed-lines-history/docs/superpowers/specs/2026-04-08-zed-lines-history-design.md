# Zed Lines History Design

## Problem

Build a GitLens-like "Lines History" experience for Zed that supports recursive line ancestry inspection:

1. The user focuses a line in the current buffer.
2. A dedicated history panel shows the current line range and its previous revision side-by-side.
3. Inside that diff view, the user can focus either side on a specific line.
4. The panel resolves the next older ancestor for that selected historical line.
5. The user can continue traversing backward revision by revision.

The defining requirement is not simple blame. It is recursive, line-anchored, cross-revision navigation.

## Hard Constraint

Current public Zed extension APIs do not expose:

- Custom dock/panel UI surface for third-party extensions.
- Live editor cursor or mouse-hover events for arbitrary extension logic.
- Editor decorations and history panes comparable to VS Code GitLens custom views.

Because of this, a marketplace-style Zed extension cannot fully implement the requested UX today.

## Decision

Use a two-layer architecture:

1. `line-history-core`
   A standalone Rust engine that computes recursive line ancestry through Git history.

2. `zed-lines-history`
   A Zed-oriented integration shell that can be used today as a CLI-driven prototype and later embedded into a private Zed fork or upstream Zed contribution once native panel/editor-event integration is available.

This preserves the hardest part, the history traversal engine, as reusable code and avoids coupling the core logic to an unavailable extension surface.

## Approaches Considered

### Approach A: Pure official Zed extension

Rejected.

Reason:
- Cannot create the required native panel UX.
- Cannot subscribe to line-focus changes from the editor in the needed way.

### Approach B: Dev extension plus external webview/tool window

Viable as a prototype.

Pros:
- Can demonstrate recursive line-history traversal.
- Can be shipped without patching Zed.

Cons:
- Does not satisfy the requested "single dedicated panel inside Zed" interaction.
- Cursor and focus synchronization remain indirect.

### Approach C: Core engine now, native Zed integration later

Selected.

Pros:
- Builds the irreversible/high-value part first.
- Keeps future UI integration thin.
- Supports both prototype mode and eventual native mode.

Cons:
- Initial deliverable is a prototype/integration scaffold, not the final in-editor panel.

## User-Facing Product Shape

### Final target shape

- A right-side or bottom `Lines History` panel in Zed.
- When the user focuses a line in the active editor, the panel shows:
  - Current revision snippet.
  - Immediate previous revision snippet for the tracked line range.
  - Metadata: commit, author, timestamp, summary.
- The panel allows clicking/focusing a line on either side and traversing backward from that chosen line.
- The panel maintains a navigation stack so the user can step back/forward in the traversal chain.

### Prototype shape for this repository

- A CLI that resolves line ancestry for `file + line`.
- Text or structured JSON output describing:
  - selected revision
  - previous revision
  - current range mapping
  - previous range mapping
  - snippet content
  - commit metadata
- The engine exposes an API designed to plug into a future native Zed panel adapter.

## Core Domain Model

### `LineAnchor`

Identifies a tracked line in a concrete revision.

Fields:
- repository root
- file path at that revision
- commit-ish
- one-based line number

### `LineRange`

Represents the smallest tracked contiguous block for display and matching.

Fields:
- start line
- end line

### `RevisionSnapshot`

Represents one side of a history step.

Fields:
- commit id
- file path
- line range
- snippet lines
- commit metadata

### `HistoryStep`

Represents one recursive edge in the traversal graph.

Fields:
- current snapshot
- previous snapshot
- mapping rationale
- confidence

### `TraversalSession`

Represents the breadcrumb chain.

Fields:
- selected anchor
- visited steps
- next candidate anchors

## Line Ancestry Resolution Strategy

The engine must do more than `git blame`.

### Step 1: Resolve owning commit for the selected line

Use `git blame --porcelain` on the target revision and file to determine:
- the commit currently responsible for the line
- the blamed line span
- original line number metadata when available

### Step 2: Materialize the current snippet

Use `git show <commit>:<path>` to load file content at the selected revision and slice a display window around the target range.

### Step 3: Find the immediate parent context

For the blamed commit, resolve its first parent for MVP.

Later expansion can support merge-parent choice.

### Step 4: Map the tracked line into the parent revision

Use patch-level line mapping from:
- `git diff -U0 <parent> <commit> -- <path>`
- or `git show --format= --unified=0 <commit> -- <path>`

Rules:
- If the target line is inside an added hunk, map to the nearest surviving parent line and mark confidence as `approximate`.
- If the line is unchanged context around the hunk, map exactly.
- If the file was renamed, detect rename path and continue with the parent path.
- If the line originated before the commit and blame points to an older origin, prefer the precise blame/origin metadata.

### Step 5: Build the next `HistoryStep`

Return:
- current revision snapshot
- mapped previous revision snapshot
- exact/approximate mapping flag

### Step 6: Repeat recursively

When the user picks a line in the previous snapshot, treat that historical snapshot as the new anchor and repeat.

## MVP Scope

Included:
- Single-file line ancestry traversal.
- First-parent history.
- Rename tracking for straightforward file renames.
- Exact or nearest-line parent mapping.
- CLI output in text and JSON.
- Rust library API with tests over synthetic Git fixture repositories.

Excluded from MVP:
- Merge-parent disambiguation UI.
- Full native Zed panel.
- Hover-based live synchronization.
- Syntax highlighting and rich diff rendering.
- Caching beyond in-process memoization.

## Repository Structure

- `Cargo.toml`
  Workspace root.
- `crates/line-history-core`
  Git traversal engine.
- `crates/zed-lines-history-cli`
  CLI prototype wrapper around the core engine.
- `tests/fixtures`
  Optional golden fixtures if needed.
- `docs/superpowers/specs/...`
  This design.
- `docs/superpowers/plans/...`
  Implementation plan.

## API Shape

The core library should expose a narrow API:

```rust
pub struct ResolveRequest {
    pub repo_root: PathBuf,
    pub rev: String,
    pub path: PathBuf,
    pub line: u32,
    pub context_before: u32,
    pub context_after: u32,
}

pub fn resolve_history_step(request: &ResolveRequest) -> Result<HistoryStep>;
```

This keeps future adapters simple:
- CLI adapter
- native Zed panel adapter
- possible language-server or MCP-facing adapter

## Error Handling

Explicit error cases:
- not a Git repository
- file missing at selected revision
- line out of range
- root commit has no parent
- binary file or non-UTF-8 content
- unsupported merge ancestry case in MVP
- unable to map line to parent revision

Errors must preserve enough detail for both CLI output and future panel messaging.

## Testing Strategy

Use test-first for the core engine.

### Unit-level fixtures

Create temporary Git repositories during tests and build exact histories:
- line unchanged across commits
- line modified in a commit
- line inserted above target
- line deleted and nearest parent line chosen
- file rename with preserved line
- root-commit edge case

### CLI tests

- JSON shape is stable.
- Text output includes current/previous commit and line ranges.

## Risks

### Risk 1: Accurate line mapping across complex diffs

Mitigation:
- Start with first-parent and nearest-line mapping.
- Expose confidence markers.

### Risk 2: Merge commits

Mitigation:
- Return explicit unsupported error for ambiguous ancestry in MVP.

### Risk 3: Zed integration remains blocked by upstream API

Mitigation:
- Keep integration boundary thin.
- Make the core independently useful.

## Future Integration Plan

When Zed exposes the needed APIs, or when building a private fork:

1. Subscribe to active editor line focus changes.
2. Maintain a `TraversalSession` keyed by panel state.
3. Render `HistoryStep` pairs inside a native panel.
4. Send click/focus events from either diff side back into the resolver.

The core engine in this repository should remain unchanged or nearly unchanged during that integration.
