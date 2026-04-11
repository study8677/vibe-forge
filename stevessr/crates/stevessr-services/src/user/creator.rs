use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};
use stevessr_core::constants::*;
use stevessr_db::models::user::User;
use stevessr_db::models::user_email::UserEmail;
use stevessr_db::models::user_profile::UserProfile;
use stevessr_db::models::user_stat::UserStat;
use stevessr_db::models::user_option::UserOption;
use ipnetwork::IpNetwork;
use regex::Regex;

pub struct CreateUserParams {
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub password: Option<String>,
    pub ip_address: Option<IpNetwork>,
    pub active: bool,
    pub approved: bool,
    pub staged: bool,
}

pub struct UserCreator;

impl UserCreator {
    pub async fn create(pool: &PgPool, params: CreateUserParams) -> Result<User> {
        Self::validate(&params)?;

        // Check username uniqueness
        if User::find_by_username(pool, &params.username).await?.is_some() {
            return Err(Error::AlreadyExists {
                resource: "user",
                detail: format!("username '{}' is taken", params.username),
            });
        }

        // Check email uniqueness
        if UserEmail::find_by_email(pool, &params.email).await?.is_some() {
            return Err(Error::AlreadyExists {
                resource: "user",
                detail: format!("email '{}' is taken", params.email),
            });
        }

        // Create user
        let user = User::create(
            pool,
            &params.username,
            params.name.as_deref(),
            params.active,
            0, // trust_level 0 (new)
            params.ip_address,
        )
        .await?;

        // Create primary email
        UserEmail::create(pool, user.id, &params.email, true).await?;

        // Create profile
        UserProfile::create(pool, user.id).await?;

        // Create stats
        UserStat::create(pool, user.id).await?;

        // Create options
        UserOption::create(pool, user.id).await?;

        // Hash and store password if provided
        if let Some(ref password) = params.password {
            Self::store_password_hash(pool, user.id, password).await?;
        }

        Ok(user)
    }

    fn validate(params: &CreateUserParams) -> Result<()> {
        let mut errors = ValidationErrors::new();

        if params.username.len() < USERNAME_MIN_LENGTH {
            errors.add("username", format!("must be at least {} characters", USERNAME_MIN_LENGTH));
        }
        if params.username.len() > USERNAME_MAX_LENGTH {
            errors.add("username", format!("must be at most {} characters", USERNAME_MAX_LENGTH));
        }
        let re = Regex::new(USERNAME_PATTERN).unwrap();
        if !re.is_match(&params.username) {
            errors.add("username", "must contain only letters, numbers, underscores, hyphens, and dots");
        }
        if params.email.is_empty() || !params.email.contains('@') {
            errors.add("email", "must be a valid email address");
        }
        if let Some(ref pw) = params.password {
            if pw.len() < 10 {
                errors.add("password", "must be at least 10 characters");
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }

    async fn store_password_hash(pool: &PgPool, user_id: i64, password: &str) -> Result<()> {
        use argon2::{Argon2, PasswordHasher};
        use argon2::password_hash::SaltString;
        use rand::rngs::OsRng;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Error::Internal(format!("password hash failed: {}", e)))?
            .to_string();

        sqlx::query("INSERT INTO user_custom_fields (user_id, name, value) VALUES ($1, 'password_hash', $2)")
            .bind(user_id)
            .bind(&hash)
            .execute(pool)
            .await?;

        Ok(())
    }
}
