use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // Authentication errors
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("account suspended until {0}")]
    AccountSuspended(chrono::DateTime<chrono::Utc>),
    #[error("account silenced until {0}")]
    AccountSilenced(chrono::DateTime<chrono::Utc>),
    #[error("account not activated")]
    AccountNotActivated,
    #[error("account not approved")]
    AccountNotApproved,
    #[error("two-factor authentication required")]
    TwoFactorRequired,
    #[error("invalid two-factor token")]
    InvalidTwoFactorToken,
    #[error("rate limited, retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },
    #[error("login locked out for {minutes} minutes")]
    LoginLockedOut { minutes: u64 },

    // Authorization errors
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("not authorized")]
    NotAuthorized,
    #[error("trust level {required} required, current: {current}")]
    InsufficientTrustLevel { required: i32, current: i32 },
    #[error("admin access required")]
    AdminRequired,
    #[error("moderator access required")]
    ModeratorRequired,
    #[error("staff access required")]
    StaffRequired,

    // Resource errors
    #[error("{resource} not found: {id}")]
    NotFound { resource: &'static str, id: String },
    #[error("{resource} already exists: {detail}")]
    AlreadyExists { resource: &'static str, detail: String },
    #[error("stale data: {resource} was modified (expected version {expected}, got {actual})")]
    StaleData { resource: &'static str, expected: i32, actual: i32 },

    // Validation errors
    #[error("validation failed: {0}")]
    Validation(#[from] ValidationErrors),
    #[error("invalid parameter: {name}: {reason}")]
    InvalidParameter { name: &'static str, reason: String },

    // Content errors
    #[error("topic is closed")]
    TopicClosed,
    #[error("topic is archived")]
    TopicArchived,
    #[error("post is locked")]
    PostLocked,
    #[error("too many edits")]
    TooManyEdits,
    #[error("spam detected")]
    SpamDetected,
    #[error("contains blocked words")]
    BlockedWords,

    // Plugin errors
    #[error("plugin error [{plugin}]: {message}")]
    Plugin { plugin: String, message: String },
    #[error("plugin not found: {0}")]
    PluginNotFound(String),
    #[error("plugin API version incompatible: required {required}, available {available}")]
    PluginApiIncompatible { required: String, available: String },

    // Infrastructure errors
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("external service error: {service}: {message}")]
    ExternalService { service: String, message: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct ValidationErrors {
    pub errors: Vec<FieldError>,
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msgs: Vec<String> = self.errors.iter().map(|e| format!("{}: {}", e.field, e.message)).collect();
        write!(f, "{}", msgs.join("; "))
    }
}

impl std::error::Error for ValidationErrors {}

impl ValidationErrors {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn add(&mut self, field: &'static str, message: impl Into<String>) {
        self.errors.push(FieldError { field, message: message.into() });
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn into_result<T>(self, value: T) -> Result<T> {
        if self.errors.is_empty() {
            Ok(value)
        } else {
            Err(Error::Validation(self))
        }
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct FieldError {
    pub field: &'static str,
    pub message: String,
}
