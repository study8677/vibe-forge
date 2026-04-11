use stevessr_core::traits::guardian::{CurrentUser, Guardian};
use stevessr_core::types::ids::*;
use stevessr_db::models::user::User;
use stevessr_db::models::group_user::GroupUser;
use sqlx::PgPool;

/// Runtime guardian implementation with database-backed permission checks.
pub struct RuntimeGuardian {
    current_user: Option<CurrentUser>,
}

impl RuntimeGuardian {
    pub fn new(user: Option<CurrentUser>) -> Self {
        Self { current_user: user }
    }

    pub async fn from_user(pool: &PgPool, user: Option<&User>) -> Result<Self, sqlx::Error> {
        let current_user = match user {
            Some(u) => {
                let groups = GroupUser::find_group_ids_for_user(pool, u.id).await?;
                Some(CurrentUser {
                    id: UserId::new(u.id),
                    username: u.username.clone(),
                    trust_level: stevessr_core::types::trust_level::TrustLevel::from_i16(u.trust_level)
                        .unwrap_or(stevessr_core::types::trust_level::TrustLevel::New),
                    admin: u.admin,
                    moderator: u.moderator,
                    silenced_till: u.silenced_till,
                    suspended_till: u.suspended_till,
                    groups: groups.into_iter().map(GroupId::new).collect(),
                })
            }
            None => None,
        };
        Ok(Self { current_user })
    }

    pub fn anonymous() -> Self {
        Self { current_user: None }
    }
}

impl Guardian for RuntimeGuardian {
    fn current_user(&self) -> Option<&CurrentUser> {
        self.current_user.as_ref()
    }
}
