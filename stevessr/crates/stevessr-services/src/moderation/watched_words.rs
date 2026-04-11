use sqlx::PgPool;
use stevessr_core::error::Result;
use regex::Regex;

/// Watched word action types.
pub const ACTION_BLOCK: i32 = 1;
pub const ACTION_CENSOR: i32 = 2;
pub const ACTION_REQUIRE_APPROVAL: i32 = 3;
pub const ACTION_FLAG: i32 = 4;
pub const ACTION_REPLACE: i32 = 5;
pub const ACTION_TAG: i32 = 6;
pub const ACTION_SILENCE: i32 = 7;
pub const ACTION_LINK: i32 = 8;

pub struct WatchedWordsChecker;

#[derive(Debug)]
pub struct WatchedWordMatch {
    pub word: String,
    pub action: i32,
    pub replacement: Option<String>,
}

impl WatchedWordsChecker {
    /// Check text against all watched words and return matches.
    pub async fn check(pool: &PgPool, text: &str) -> Result<Vec<WatchedWordMatch>> {
        let words: Vec<(String, i32, Option<String>, bool)> = sqlx::query_as(
            "SELECT word, action, replacement, case_sensitive FROM watched_words ORDER BY action"
        )
        .fetch_all(pool)
        .await?;

        let mut matches = Vec::new();
        let text_lower = text.to_lowercase();

        for (word, action, replacement, case_sensitive) in words {
            let found = if case_sensitive {
                text.contains(&word)
            } else {
                text_lower.contains(&word.to_lowercase())
            };

            if found {
                matches.push(WatchedWordMatch {
                    word,
                    action,
                    replacement,
                });
            }
        }

        Ok(matches)
    }

    /// Apply watched word replacements to text.
    pub async fn apply_censoring(pool: &PgPool, text: &str) -> Result<String> {
        let words: Vec<(String, Option<String>, bool)> = sqlx::query_as(
            "SELECT word, replacement, case_sensitive FROM watched_words WHERE action = $1"
        )
        .bind(ACTION_CENSOR)
        .fetch_all(pool)
        .await?;

        let mut result = text.to_string();

        for (word, _replacement, case_sensitive) in words {
            let pattern = if case_sensitive {
                format!(r"\b{}\b", regex::escape(&word))
            } else {
                format!(r"(?i)\b{}\b", regex::escape(&word))
            };

            if let Ok(re) = Regex::new(&pattern) {
                let censored = "*".repeat(word.len());
                result = re.replace_all(&result, censored.as_str()).to_string();
            }
        }

        Ok(result)
    }

    /// Check if text should be blocked due to watched words.
    pub async fn should_block(pool: &PgPool, text: &str) -> Result<Option<String>> {
        let matches = Self::check(pool, text).await?;
        for m in matches {
            if m.action == ACTION_BLOCK {
                return Ok(Some(m.word));
            }
        }
        Ok(None)
    }

    /// Add a new watched word.
    pub async fn add_word(pool: &PgPool, word: &str, action: i32, replacement: Option<&str>, case_sensitive: bool) -> Result<i64> {
        let row: (i64,) = sqlx::query_as(
            "INSERT INTO watched_words (word, action, replacement, case_sensitive, created_at, updated_at)
             VALUES ($1, $2, $3, $4, NOW(), NOW())
             ON CONFLICT (word, action) DO UPDATE SET replacement = $3, case_sensitive = $4, updated_at = NOW()
             RETURNING id"
        )
        .bind(word)
        .bind(action)
        .bind(replacement)
        .bind(case_sensitive)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    pub async fn remove_word(pool: &PgPool, word_id: i64) -> Result<()> {
        sqlx::query("DELETE FROM watched_words WHERE id = $1")
            .bind(word_id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
