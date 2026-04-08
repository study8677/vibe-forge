# Zed Lines History Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a reusable Rust engine that resolves recursive Git line ancestry and expose it through a CLI prototype shaped for later Zed native-panel integration.

**Architecture:** The project is split into a pure core crate and a thin CLI adapter. The core owns Git plumbing, line mapping, domain types, and errors. The CLI owns argument parsing and serialization only.

**Tech Stack:** Rust workspace, `git` CLI plumbing, `tempfile`, `serde`, `serde_json`, `clap`, `anyhow` or narrow custom error types.

---

### Task 1: Scaffold workspace

**Files:**
- Create: `Cargo.toml`
- Create: `crates/line-history-core/Cargo.toml`
- Create: `crates/line-history-core/src/lib.rs`
- Create: `crates/zed-lines-history-cli/Cargo.toml`
- Create: `crates/zed-lines-history-cli/src/main.rs`
- Create: `.gitignore`
- Create: `README.md`

- [ ] **Step 1: Create workspace manifest**

```toml
[workspace]
members = ["crates/line-history-core", "crates/zed-lines-history-cli"]
resolver = "2"
```

- [ ] **Step 2: Create the core crate manifest**

```toml
[package]
name = "line-history-core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
thiserror = "1"
```

- [ ] **Step 3: Create the CLI crate manifest**

```toml
[package]
name = "zed-lines-history-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }
line-history-core = { path = "../line-history-core" }
serde_json = "1"
```

- [ ] **Step 4: Create placeholder compileable sources**

```rust
// crates/line-history-core/src/lib.rs
pub fn placeholder() -> &'static str {
    "placeholder"
}
```

```rust
// crates/zed-lines-history-cli/src/main.rs
fn main() {
    println!("{}", line_history_core::placeholder());
}
```

- [ ] **Step 5: Verify the workspace compiles**

Run: `cargo check`
Expected: PASS

### Task 2: Define domain model and failing tests

**Files:**
- Modify: `crates/line-history-core/Cargo.toml`
- Modify: `crates/line-history-core/src/lib.rs`
- Create: `crates/line-history-core/tests/resolve_history_step.rs`

- [ ] **Step 1: Add test dependency**

```toml
[dev-dependencies]
tempfile = "3"
```

- [ ] **Step 2: Write the first failing integration test for unchanged-line ancestry**

```rust
#[test]
fn resolves_previous_snapshot_for_an_unchanged_line() {
    // create temp repo, commit file twice with unrelated change
    // request history step for a stable line
    // assert current commit id != previous commit id
    // assert previous snapshot contains the same logical line text
}
```

- [ ] **Step 3: Run the test to confirm it fails**

Run: `cargo test -p line-history-core resolves_previous_snapshot_for_an_unchanged_line -- --exact`
Expected: FAIL because resolver API is not implemented

- [ ] **Step 4: Define core public types in `src/lib.rs`**

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolveRequest { /* fields */ }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryStep { /* fields */ }

pub fn resolve_history_step(_request: &ResolveRequest) -> Result<HistoryStep, LineHistoryError> {
    todo!()
}
```

- [ ] **Step 5: Re-run the same test**

Run: `cargo test -p line-history-core resolves_previous_snapshot_for_an_unchanged_line -- --exact`
Expected: FAIL with `todo!` or explicit unimplemented error

### Task 3: Implement Git command runner and repository fixture helpers

**Files:**
- Modify: `crates/line-history-core/src/lib.rs`
- Modify: `crates/line-history-core/tests/resolve_history_step.rs`

- [ ] **Step 1: Add a failing test helper that initializes a temp git repo and commits content**

```rust
fn git(repo: &Path, args: &[&str]) -> String {
    // run git command and return stdout
}
```

- [ ] **Step 2: Run tests to confirm helper assumptions fail if missing**

Run: `cargo test -p line-history-core resolves_previous_snapshot_for_an_unchanged_line -- --exact`
Expected: FAIL before resolver logic, if helper is incomplete

- [ ] **Step 3: Implement a minimal internal command runner**

```rust
fn run_git(repo_root: &Path, args: &[&str]) -> Result<String, LineHistoryError> {
    // std::process::Command::new("git")
}
```

- [ ] **Step 4: Make tests green up to the resolver boundary**

Run: `cargo test -p line-history-core resolves_previous_snapshot_for_an_unchanged_line -- --exact`
Expected: FAIL only because `resolve_history_step` is not implemented

### Task 4: Implement first passing resolver for blame + parent content

**Files:**
- Modify: `crates/line-history-core/src/lib.rs`
- Modify: `crates/line-history-core/tests/resolve_history_step.rs`

- [ ] **Step 1: Write a failing assertion for commit metadata and snapshot text**

```rust
assert_eq!(step.current.file_path, PathBuf::from("src/demo.rs"));
assert!(step.current.snippet.iter().any(|line| line.text.contains("tracked_line")));
assert!(!step.previous.commit_id.is_empty());
```

- [ ] **Step 2: Run the test to confirm failure**

Run: `cargo test -p line-history-core resolves_previous_snapshot_for_an_unchanged_line -- --exact`
Expected: FAIL on missing resolver behavior

- [ ] **Step 3: Implement minimal resolver path**

```rust
pub fn resolve_history_step(request: &ResolveRequest) -> Result<HistoryStep, LineHistoryError> {
    // rev-parse request.rev
    // blame selected line
    // rev-parse commit^
    // show current and parent file contents
    // build snapshots with a single-line range for MVP
}
```

- [ ] **Step 4: Re-run the test**

Run: `cargo test -p line-history-core resolves_previous_snapshot_for_an_unchanged_line -- --exact`
Expected: PASS

### Task 5: Add diff-based line mapping tests

**Files:**
- Modify: `crates/line-history-core/tests/resolve_history_step.rs`
- Modify: `crates/line-history-core/src/lib.rs`

- [ ] **Step 1: Add a failing test for line moved by insertion above**

```rust
#[test]
fn maps_line_to_parent_when_lines_are_inserted_above_it() {
    // commit 1 baseline
    // commit 2 inserts lines above the tracked line
    // tracked logical line should map to parent line minus inserted count
}
```

- [ ] **Step 2: Run the new test**

Run: `cargo test -p line-history-core maps_line_to_parent_when_lines_are_inserted_above_it -- --exact`
Expected: FAIL

- [ ] **Step 3: Implement zero-context diff hunk parsing**

```rust
fn map_line_to_parent(/* current line, diff text */) -> LineMapping {
    // parse @@ -a,b +c,d @@ hunks
}
```

- [ ] **Step 4: Re-run the test**

Run: `cargo test -p line-history-core maps_line_to_parent_when_lines_are_inserted_above_it -- --exact`
Expected: PASS

### Task 6: Add changed-line and deleted-line edge cases

**Files:**
- Modify: `crates/line-history-core/tests/resolve_history_step.rs`
- Modify: `crates/line-history-core/src/lib.rs`

- [ ] **Step 1: Add a failing test for a line modified in the selected commit**

```rust
#[test]
fn returns_previous_line_content_for_a_modified_line() {
    // same logical location, different text in parent
}
```

- [ ] **Step 2: Add a failing test for deleted-line nearest-parent fallback**

```rust
#[test]
fn marks_mapping_as_approximate_when_exact_parent_line_is_deleted() {
    // selected line lands in added hunk
}
```

- [ ] **Step 3: Run both tests**

Run: `cargo test -p line-history-core returns_previous_line_content_for_a_modified_line marks_mapping_as_approximate_when_exact_parent_line_is_deleted`
Expected: FAIL

- [ ] **Step 4: Implement mapping confidence and nearest-parent fallback**

```rust
pub enum MappingConfidence {
    Exact,
    Approximate,
}
```

- [ ] **Step 5: Re-run the targeted tests**

Run: `cargo test -p line-history-core returns_previous_line_content_for_a_modified_line marks_mapping_as_approximate_when_exact_parent_line_is_deleted`
Expected: PASS

### Task 7: Add rename tracking

**Files:**
- Modify: `crates/line-history-core/tests/resolve_history_step.rs`
- Modify: `crates/line-history-core/src/lib.rs`

- [ ] **Step 1: Add a failing test for a renamed file**

```rust
#[test]
fn follows_simple_file_rename_into_parent_history() {
    // file renamed between parent and child commit
}
```

- [ ] **Step 2: Run the test**

Run: `cargo test -p line-history-core follows_simple_file_rename_into_parent_history -- --exact`
Expected: FAIL

- [ ] **Step 3: Implement rename lookup**

```rust
fn resolve_parent_path_for_commit(/* parent, child, path */) -> Result<PathBuf, LineHistoryError> {
    // git diff --name-status parent child -- path
}
```

- [ ] **Step 4: Re-run the test**

Run: `cargo test -p line-history-core follows_simple_file_rename_into_parent_history -- --exact`
Expected: PASS

### Task 8: Build CLI adapter

**Files:**
- Modify: `crates/zed-lines-history-cli/src/main.rs`
- Modify: `crates/zed-lines-history-cli/Cargo.toml`
- Test: `cargo run -p zed-lines-history-cli -- --help`

- [ ] **Step 1: Write a failing smoke expectation for CLI JSON output**

```text
zed-lines-history-cli --repo . --rev HEAD --path src/demo.rs --line 12 --format json
```

- [ ] **Step 2: Run the CLI help command**

Run: `cargo run -p zed-lines-history-cli -- --help`
Expected: FAIL if args are not implemented

- [ ] **Step 3: Implement `clap` argument parsing and JSON/text output**

```rust
#[derive(clap::Parser)]
struct Args {
    #[arg(long)]
    repo: PathBuf,
    #[arg(long, default_value = "HEAD")]
    rev: String,
    #[arg(long)]
    path: PathBuf,
    #[arg(long)]
    line: u32,
    #[arg(long, default_value = "text")]
    format: String,
}
```

- [ ] **Step 4: Re-run help and a manual local invocation**

Run: `cargo run -p zed-lines-history-cli -- --help`
Expected: PASS

### Task 9: Finish docs

**Files:**
- Modify: `README.md`
- Modify: `docs/superpowers/specs/2026-04-08-zed-lines-history-design.md`

- [ ] **Step 1: Document the hard Zed extension limitation and chosen architecture**

```md
## Why this is not a pure marketplace extension
...
```

- [ ] **Step 2: Document how to run the CLI prototype**

```md
cargo run -p zed-lines-history-cli -- \
  --repo /path/to/repo \
  --rev HEAD \
  --path src/lib.rs \
  --line 42 \
  --format text
```

- [ ] **Step 3: Verify docs align with actual commands**

Run: `cargo run -p zed-lines-history-cli -- --help`
Expected: PASS

### Task 10: Full verification

**Files:**
- Test: `cargo fmt --all --check`
- Test: `cargo clippy --workspace --all-targets -- -D warnings`
- Test: `cargo test --workspace`

- [ ] **Step 1: Run formatting check**

Run: `cargo fmt --all --check`
Expected: PASS

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --workspace --all-targets -- -D warnings`
Expected: PASS

- [ ] **Step 3: Run all tests**

Run: `cargo test --workspace`
Expected: PASS
