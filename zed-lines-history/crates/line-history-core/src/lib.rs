use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveRequest {
    pub repo_root: PathBuf,
    pub rev: String,
    pub path: PathBuf,
    pub line: u32,
    pub context_before: u32,
    pub context_after: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryStep {
    pub current: RevisionSnapshot,
    pub previous: RevisionSnapshot,
    pub mapping_confidence: MappingConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionSnapshot {
    pub commit_id: String,
    pub file_path: PathBuf,
    pub line_range: LineRange,
    pub snippet: Vec<SnippetLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetLine {
    pub number: u32,
    pub text: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MappingConfidence {
    Exact,
    Approximate,
}

#[derive(Debug, Error)]
pub enum LineHistoryError {
    #[error("git command failed: git {command}\n{stderr}")]
    GitCommandFailed { command: String, stderr: String },
    #[error("revision `{0}` does not have a parent commit")]
    NoParentCommit(String),
    #[error("line {line} is out of range for `{path}`")]
    LineOutOfRange { path: PathBuf, line: u32 },
    #[error("could not parse diff hunk header: {0}")]
    DiffParse(String),
    #[error("file `{path}` not found at revision `{rev}`")]
    FileNotFound { rev: String, path: PathBuf },
}

pub fn resolve_history_step(request: &ResolveRequest) -> Result<HistoryStep, LineHistoryError> {
    let current_commit = run_git(&request.repo_root, &["rev-parse", request.rev.as_str()])?;
    let parent_commit = run_git(
        &request.repo_root,
        &["rev-parse", &format!("{current_commit}^")],
    )
    .map_err(|_| LineHistoryError::NoParentCommit(current_commit.clone()))?;

    let current_path = request.path.clone();
    let previous_path = resolve_previous_path(
        &request.repo_root,
        &parent_commit,
        &current_commit,
        &current_path,
    )?;

    let current_lines = show_file_at_revision(&request.repo_root, &current_commit, &current_path)?;
    let previous_lines = show_file_at_revision(&request.repo_root, &parent_commit, &previous_path)?;

    validate_line_in_range(&current_path, request.line, &current_lines)?;
    let mapping = if previous_path != current_path {
        LineMapping {
            parent_line: request.line,
            confidence: MappingConfidence::Exact,
        }
    } else {
        map_line_to_parent(
            &request.repo_root,
            &parent_commit,
            &current_commit,
            &current_path,
            request.line,
        )?
    };
    validate_line_in_range(&previous_path, mapping.parent_line, &previous_lines)?;

    Ok(HistoryStep {
        current: build_snapshot(
            current_commit,
            current_path,
            request.line,
            request.context_before,
            request.context_after,
            &current_lines,
        ),
        previous: build_snapshot(
            parent_commit,
            previous_path,
            mapping.parent_line,
            request.context_before,
            request.context_after,
            &previous_lines,
        ),
        mapping_confidence: mapping.confidence,
    })
}

fn validate_line_in_range(
    path: &Path,
    line: u32,
    lines: &[String],
) -> Result<(), LineHistoryError> {
    if line == 0 || line as usize > lines.len() {
        return Err(LineHistoryError::LineOutOfRange {
            path: path.to_path_buf(),
            line,
        });
    }

    Ok(())
}

fn show_file_at_revision(
    repo_root: &PathBuf,
    commit: &str,
    path: &Path,
) -> Result<Vec<String>, LineHistoryError> {
    let path_str = path.to_string_lossy();
    let content = run_git(repo_root, &["show", &format!("{commit}:{path_str}")]).map_err(|_| {
        LineHistoryError::FileNotFound {
            rev: commit.to_string(),
            path: path.to_path_buf(),
        }
    })?;

    Ok(content.lines().map(ToOwned::to_owned).collect())
}

fn build_snapshot(
    commit_id: String,
    file_path: PathBuf,
    focus_line: u32,
    context_before: u32,
    context_after: u32,
    file_lines: &[String],
) -> RevisionSnapshot {
    let start = focus_line.saturating_sub(context_before).max(1);
    let end = (focus_line + context_after).min(file_lines.len() as u32);
    let snippet = (start..=end)
        .map(|line_number| SnippetLine {
            number: line_number,
            text: file_lines[(line_number - 1) as usize].clone(),
        })
        .collect();

    RevisionSnapshot {
        commit_id,
        file_path,
        line_range: LineRange {
            start: focus_line,
            end: focus_line,
        },
        snippet,
    }
}

fn resolve_previous_path(
    repo_root: &PathBuf,
    parent_commit: &str,
    current_commit: &str,
    current_path: &Path,
) -> Result<PathBuf, LineHistoryError> {
    let current_path_str = current_path.to_string_lossy();
    let output = run_git(
        repo_root,
        &["diff", "--name-status", "-M", parent_commit, current_commit],
    )?;

    for line in output.lines() {
        let mut parts = line.split_whitespace();
        let Some(status) = parts.next() else {
            continue;
        };

        if status.starts_with('R') {
            let Some(old_path) = parts.next() else {
                continue;
            };
            let Some(new_path) = parts.next() else {
                continue;
            };
            if new_path == current_path_str {
                return Ok(PathBuf::from(old_path));
            }
        }
    }

    Ok(current_path.to_path_buf())
}

struct LineMapping {
    parent_line: u32,
    confidence: MappingConfidence,
}

fn map_line_to_parent(
    repo_root: &PathBuf,
    parent_commit: &str,
    current_commit: &str,
    path: &Path,
    current_line: u32,
) -> Result<LineMapping, LineHistoryError> {
    let path_str = path.to_string_lossy();
    let diff = run_git(
        repo_root,
        &[
            "diff",
            "--unified=0",
            "--no-color",
            parent_commit,
            current_commit,
            "--",
            &path_str,
        ],
    )?;

    let mut line_delta: i64 = 0;
    for hunk in parse_hunks(&diff)? {
        if hunk.new_count == 0 {
            if current_line >= hunk.new_start {
                line_delta += i64::from(hunk.old_count);
            }
            continue;
        }

        let new_end = hunk.new_start + hunk.new_count - 1;
        if current_line < hunk.new_start {
            break;
        }

        if current_line > new_end {
            line_delta += i64::from(hunk.old_count) - i64::from(hunk.new_count);
            continue;
        }

        if hunk.old_count == 0 {
            return Ok(LineMapping {
                parent_line: hunk.old_start.max(1),
                confidence: MappingConfidence::Approximate,
            });
        }

        let relative = current_line - hunk.new_start;
        let mapped = hunk.old_start + relative.min(hunk.old_count - 1);
        let confidence = if hunk.old_count == hunk.new_count {
            MappingConfidence::Exact
        } else {
            MappingConfidence::Approximate
        };

        return Ok(LineMapping {
            parent_line: mapped.max(1),
            confidence,
        });
    }

    Ok(LineMapping {
        parent_line: (i64::from(current_line) + line_delta).max(1) as u32,
        confidence: MappingConfidence::Exact,
    })
}

#[derive(Debug, Clone, Copy)]
struct DiffHunk {
    old_start: u32,
    old_count: u32,
    new_start: u32,
    new_count: u32,
}

fn parse_hunks(diff: &str) -> Result<Vec<DiffHunk>, LineHistoryError> {
    let mut hunks = Vec::new();

    for line in diff.lines() {
        if !line.starts_with("@@") {
            continue;
        }

        let Some((header, _)) = line[3..].split_once("@@") else {
            return Err(LineHistoryError::DiffParse(line.to_string()));
        };
        let mut parts = header.split_whitespace();
        let old_part = parts
            .next()
            .ok_or_else(|| LineHistoryError::DiffParse(line.to_string()))?;
        let new_part = parts
            .next()
            .ok_or_else(|| LineHistoryError::DiffParse(line.to_string()))?;

        let (old_start, old_count) = parse_range(old_part, '-')?;
        let (new_start, new_count) = parse_range(new_part, '+')?;
        hunks.push(DiffHunk {
            old_start,
            old_count,
            new_start,
            new_count,
        });
    }

    Ok(hunks)
}

fn parse_range(part: &str, marker: char) -> Result<(u32, u32), LineHistoryError> {
    let range = part
        .strip_prefix(marker)
        .ok_or_else(|| LineHistoryError::DiffParse(part.to_string()))?;
    let Some((start, count)) = range.split_once(',') else {
        let start = range
            .parse::<u32>()
            .map_err(|_| LineHistoryError::DiffParse(part.to_string()))?;
        return Ok((start, 1));
    };

    let start = start
        .parse::<u32>()
        .map_err(|_| LineHistoryError::DiffParse(part.to_string()))?;
    let count = count
        .parse::<u32>()
        .map_err(|_| LineHistoryError::DiffParse(part.to_string()))?;
    Ok((start, count))
}

fn run_git(repo_root: &PathBuf, args: &[&str]) -> Result<String, LineHistoryError> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_root)
        .output()
        .map_err(|error| LineHistoryError::GitCommandFailed {
            command: args.join(" "),
            stderr: error.to_string(),
        })?;

    if !output.status.success() {
        return Err(LineHistoryError::GitCommandFailed {
            command: args.join(" "),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string())
}
