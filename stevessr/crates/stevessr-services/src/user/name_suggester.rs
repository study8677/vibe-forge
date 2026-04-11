use sqlx::PgPool;
use stevessr_db::models::user::User;

pub struct NameSuggester;

impl NameSuggester {
    pub async fn suggest(pool: &PgPool, base: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        let clean = base.chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>();

        for suffix in ["", "1", "2", "3", "11", "22", "33"] {
            let candidate = format!("{}{}", clean, suffix);
            if candidate.len() >= 3 {
                if User::find_by_username(pool, &candidate).await.ok().flatten().is_none() {
                    suggestions.push(candidate);
                }
            }
            if suggestions.len() >= 3 {
                break;
            }
        }

        suggestions
    }
}
