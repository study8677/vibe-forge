use sqlx::PgPool;
use stevessr_core::error::Result;
use stevessr_db::models::user::User;
use stevessr_db::models::user_stat::UserStat;
use stevessr_core::constants::*;

pub struct TrustLevelManager;

impl TrustLevelManager {
    pub async fn grant(pool: &PgPool, user_id: i64, level: i16) -> Result<()> {
        User::set_trust_level(pool, user_id, level).await?;
        Ok(())
    }

    pub async fn check_and_promote(pool: &PgPool, user_id: i64) -> Result<Option<i16>> {
        let user = User::find_by_id(pool, user_id).await?.ok_or(stevessr_core::error::Error::NotFound {
            resource: "user", id: user_id.to_string(),
        })?;
        let stats = UserStat::find_by_user_id(pool, user_id).await?.ok_or(stevessr_core::error::Error::NotFound {
            resource: "user_stat", id: user_id.to_string(),
        })?;

        let new_level = match user.trust_level {
            0 => {
                if stats.topics_entered >= TL1_REQUIRES_TOPICS_ENTERED
                    && stats.posts_read_count >= TL1_REQUIRES_POSTS_READ
                    && stats.time_read >= (TL1_REQUIRES_TIME_SPENT_MINS * 60) as i64
                {
                    Some(1)
                } else {
                    None
                }
            }
            1 => {
                if stats.topics_entered >= TL2_REQUIRES_TOPICS_ENTERED
                    && stats.posts_read_count >= TL2_REQUIRES_POSTS_READ
                    && stats.time_read >= (TL2_REQUIRES_TIME_SPENT_MINS * 60) as i64
                    && stats.days_visited >= TL2_REQUIRES_DAYS_VISITED
                    && stats.likes_given >= TL2_REQUIRES_LIKES_GIVEN
                    && stats.likes_received >= TL2_REQUIRES_LIKES_RECEIVED
                {
                    Some(2)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(level) = new_level {
            Self::grant(pool, user_id, level).await?;
        }

        Ok(new_level)
    }
}
