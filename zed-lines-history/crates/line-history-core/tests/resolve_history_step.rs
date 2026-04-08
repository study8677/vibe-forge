use std::fs;
use std::path::Path;
use std::process::Command;

use line_history_core::{resolve_history_step, MappingConfidence, ResolveRequest};
use tempfile::TempDir;

#[test]
fn resolves_previous_snapshot_for_an_unchanged_line() {
    let repo = TempDir::new().expect("create temp repo");
    git(repo.path(), &["init"]);
    git(repo.path(), &["config", "user.name", "Test User"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);

    write_file(
        repo.path(),
        "src/demo.rs",
        "fn main() {\n    let tracked_line = 1;\n    println!(\"{}\", tracked_line);\n}\n",
    );
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-m", "initial"]);

    write_file(
        repo.path(),
        "src/demo.rs",
        "fn main() {\n    let tracked_line = 1;\n    println!(\"{}\", tracked_line);\n    println!(\"new line\");\n}\n",
    );
    git(repo.path(), &["add", "."]);
    git(repo.path(), &["commit", "-m", "add unrelated line"]);

    let step = resolve_history_step(&ResolveRequest {
        repo_root: repo.path().to_path_buf(),
        rev: "HEAD".to_string(),
        path: "src/demo.rs".into(),
        line: 2,
        context_before: 1,
        context_after: 1,
    })
    .expect("resolve line history");

    assert_ne!(step.current.commit_id, step.previous.commit_id);
    assert_eq!(step.current.line_range.start, 2);
    assert_eq!(step.previous.line_range.start, 2);
    assert!(
        step.previous
            .snippet
            .iter()
            .any(|line| line.text.contains("tracked_line = 1")),
        "expected previous snapshot to include the tracked line"
    );
}

#[test]
fn maps_line_to_parent_when_lines_are_inserted_above_it() {
    let repo = TempDir::new().expect("create temp repo");
    init_repo(repo.path());

    write_file(repo.path(), "src/demo.rs", "alpha\ntracked\nomega\n");
    commit_all(repo.path(), "initial");

    write_file(repo.path(), "src/demo.rs", "intro\nalpha\ntracked\nomega\n");
    commit_all(repo.path(), "insert above");

    let step = resolve(repo.path(), "HEAD", "src/demo.rs", 3);

    assert_eq!(step.current.line_range.start, 3);
    assert_eq!(step.previous.line_range.start, 2);
    assert_eq!(step.mapping_confidence, MappingConfidence::Exact);
}

#[test]
fn returns_previous_line_content_for_a_modified_line() {
    let repo = TempDir::new().expect("create temp repo");
    init_repo(repo.path());

    write_file(
        repo.path(),
        "src/demo.rs",
        "fn main() {\n    let tracked_line = 1;\n}\n",
    );
    commit_all(repo.path(), "initial");

    write_file(
        repo.path(),
        "src/demo.rs",
        "fn main() {\n    let tracked_line = 2;\n}\n",
    );
    commit_all(repo.path(), "modify tracked line");

    let step = resolve(repo.path(), "HEAD", "src/demo.rs", 2);

    assert_eq!(step.previous.line_range.start, 2);
    assert_eq!(step.mapping_confidence, MappingConfidence::Exact);
    assert!(
        step.previous
            .snippet
            .iter()
            .any(|line| line.text.contains("tracked_line = 1")),
        "expected previous snapshot to include the old line content"
    );
}

#[test]
fn marks_mapping_as_approximate_when_current_line_was_added_in_selected_revision() {
    let repo = TempDir::new().expect("create temp repo");
    init_repo(repo.path());

    write_file(repo.path(), "src/demo.rs", "alpha\nomega\n");
    commit_all(repo.path(), "initial");

    write_file(repo.path(), "src/demo.rs", "new-top-line\nalpha\nomega\n");
    commit_all(repo.path(), "add new top line");

    let step = resolve(repo.path(), "HEAD", "src/demo.rs", 1);

    assert_eq!(step.mapping_confidence, MappingConfidence::Approximate);
    assert_eq!(step.previous.line_range.start, 1);
    assert!(
        step.previous
            .snippet
            .iter()
            .any(|line| line.text == "alpha"),
        "expected fallback to nearest parent context"
    );
}

#[test]
fn follows_simple_file_rename_into_parent_history() {
    let repo = TempDir::new().expect("create temp repo");
    init_repo(repo.path());

    write_file(repo.path(), "src/old_name.rs", "alpha\ntracked\nomega\n");
    commit_all(repo.path(), "initial");

    let old_path = repo.path().join("src/old_name.rs");
    let new_path = repo.path().join("src/new_name.rs");
    fs::rename(old_path, new_path).expect("rename file");
    git(repo.path(), &["add", "-A"]);
    git(repo.path(), &["commit", "-m", "rename file"]);

    let step = resolve(repo.path(), "HEAD", "src/new_name.rs", 2);

    assert_eq!(
        step.previous.file_path,
        std::path::PathBuf::from("src/old_name.rs")
    );
    assert_eq!(step.previous.line_range.start, 2);
}

#[test]
fn returns_no_parent_error_for_root_commit() {
    let repo = TempDir::new().expect("create temp repo");
    init_repo(repo.path());

    write_file(repo.path(), "src/demo.rs", "alpha\n");
    commit_all(repo.path(), "initial");

    let error = resolve_history_step(&ResolveRequest {
        repo_root: repo.path().to_path_buf(),
        rev: "HEAD".to_string(),
        path: "src/demo.rs".into(),
        line: 1,
        context_before: 0,
        context_after: 0,
    })
    .expect_err("root commit should not have a parent");

    assert!(
        error.to_string().contains("does not have a parent"),
        "unexpected error: {error}"
    );
}

fn resolve(repo_root: &Path, rev: &str, path: &str, line: u32) -> line_history_core::HistoryStep {
    resolve_history_step(&ResolveRequest {
        repo_root: repo_root.to_path_buf(),
        rev: rev.to_string(),
        path: path.into(),
        line,
        context_before: 1,
        context_after: 1,
    })
    .expect("resolve line history")
}

fn init_repo(repo_root: &Path) {
    git(repo_root, &["init"]);
    git(repo_root, &["config", "user.name", "Test User"]);
    git(repo_root, &["config", "user.email", "test@example.com"]);
}

fn commit_all(repo_root: &Path, message: &str) {
    git(repo_root, &["add", "."]);
    git(repo_root, &["commit", "-m", message]);
}

fn write_file(repo_root: &Path, relative_path: &str, contents: &str) {
    let path = repo_root.join(relative_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, contents).expect("write file");
}

fn git(repo_root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .expect("run git command");

    if !output.status.success() {
        panic!(
            "git command failed: git {}\nstdout:\n{}\nstderr:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
