use async_trait::async_trait;
use axum::http::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// Target serializers that plugins can extend with additional fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SerializerTarget {
    User,
    Topic,
    TopicList,
    Post,
    Category,
    Group,
    Tag,
    Notification,
    Badge,
    SearchResult,
    ChatChannel,
    ChatMessage,
    Reviewable,
    Site,
    Admin,
}

/// Target models for callbacks and validators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelTarget {
    User,
    Topic,
    Post,
    Category,
    Group,
    Tag,
    Badge,
    Notification,
    Upload,
    ChatChannel,
    ChatMessage,
    Bookmark,
    Invite,
}

/// Callback types for model lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CallbackType {
    BeforeCreate,
    AfterCreate,
    BeforeSave,
    AfterSave,
    BeforeDestroy,
    AfterDestroy,
}

/// Middleware injection position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiddlewarePosition {
    /// Runs before authentication.
    BeforeAuth,
    /// Runs after authentication, before route handlers.
    AfterAuth,
    /// Runs after route handlers, before response.
    AfterHandler,
}

/// Extracts additional fields for a serializer.
#[async_trait]
pub trait SerializerFieldExtractor: Send + Sync {
    async fn extract(&self, entity_id: i64, context: &Value) -> Option<Value>;
}

/// Handles a custom reviewable type.
#[async_trait]
pub trait ReviewableHandler: Send + Sync {
    async fn perform_action(&self, reviewable_id: i64, action: &str, actor_id: i64) -> Result<(), crate::PluginError>;
    fn available_actions(&self) -> Vec<String>;
}

/// Handles a custom bookmarkable type.
#[async_trait]
pub trait BookmarkableHandler: Send + Sync {
    async fn resolve_url(&self, bookmarkable_id: i64) -> Option<String>;
    async fn resolve_title(&self, bookmarkable_id: i64) -> Option<String>;
}

/// Generates a custom admin report.
#[async_trait]
pub trait ReportGenerator: Send + Sync {
    async fn generate(&self, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Result<Value, crate::PluginError>;
    fn report_type(&self) -> &str;
    fn title(&self) -> &str;
    fn description(&self) -> &str;
}

/// Custom hashtag data source.
#[async_trait]
pub trait HashtagSource: Send + Sync {
    async fn search(&self, query: &str, limit: usize) -> Vec<HashtagResult>;
    fn source_type(&self) -> &str;
    fn icon(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashtagResult {
    pub id: i64,
    pub text: String,
    pub slug: String,
    pub relative_url: String,
    pub icon: String,
    pub source_type: String,
}

/// Custom search filter.
#[async_trait]
pub trait SearchFilter: Send + Sync {
    fn key(&self) -> &str;
    async fn apply(&self, query: &str, value: &str) -> Option<String>;
}

/// Presence channel handler.
#[async_trait]
pub trait PresenceHandler: Send + Sync {
    async fn can_join(&self, user_id: i64, channel: &str) -> bool;
}

/// Directory column definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryColumnDefinition {
    pub name: String,
    pub label: String,
    pub sql_expression: String,
    pub sortable: bool,
}

/// API scope matcher for custom scopes.
pub type ApiScopeMatcher = Arc<dyn Fn(&str, &str) -> bool + Send + Sync>;

/// Rate limiter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterDefinition {
    pub key: String,
    pub max_requests: u32,
    pub period_seconds: u64,
}

/// Admin route definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRouteDefinition {
    pub label: String,
    pub location: String,
}

/// Badge trigger definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeTriggerDefinition {
    pub trigger_type: i32,
    pub name: String,
}

/// Job definition for plugin-registered background jobs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDefinition {
    pub name: String,
    pub queue: String,
}

/// Custom field definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldDefinition {
    pub name: String,
    pub field_type: String,
    pub max_length: Option<usize>,
    pub filterable: bool,
    pub searchable: bool,
}

/// Modifier for named modification points.
#[async_trait]
pub trait Modifier: Send + Sync {
    async fn modify(&self, value: Value) -> Value;
}

/// Model callback.
#[async_trait]
pub trait ModelCallback: Send + Sync {
    async fn execute(&self, model_data: &Value) -> Result<(), crate::PluginError>;
}

/// Validator for model data.
#[async_trait]
pub trait Validator: Send + Sync {
    async fn validate(&self, model_data: &Value) -> Result<(), Vec<String>>;
}

/// Cron schedule expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub expression: String,
}

/// Boxed HTTP handler for plugin routes.
pub type BoxedHandler = Box<dyn Fn() + Send + Sync>; // Placeholder, actual Axum handler

/// Boxed Tower layer for middleware injection.
pub type BoxedLayer = Box<dyn std::any::Any + Send + Sync>; // Placeholder

/// Setting definition for dynamic site settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteSettingDefinition {
    pub name: String,
    pub setting_type: String,
    pub default: Value,
    pub description: String,
    pub category: String,
}

/// The extension registry trait. Plugins call these during configure().
pub trait ExtensionRegistry: Send + Sync {
    fn register_route(&mut self, method: Method, path: &str, handler: BoxedHandler);
    fn register_serializer_field(&mut self, target: SerializerTarget, field_name: &str, extractor: Box<dyn SerializerFieldExtractor>);
    fn register_site_setting(&mut self, setting: SiteSettingDefinition);
    fn register_user_custom_field(&mut self, field: CustomFieldDefinition);
    fn register_topic_custom_field(&mut self, field: CustomFieldDefinition);
    fn register_post_custom_field(&mut self, field: CustomFieldDefinition);
    fn register_notification_type(&mut self, type_name: &str, id: i32);
    fn register_reviewable_type(&mut self, type_name: &str, handler: Box<dyn ReviewableHandler>);
    fn register_bookmarkable_type(&mut self, type_name: &str, handler: Box<dyn BookmarkableHandler>);
    fn register_badge_trigger(&mut self, trigger: BadgeTriggerDefinition);
    fn register_admin_route(&mut self, route: AdminRouteDefinition);
    fn register_report(&mut self, name: &str, generator: Box<dyn ReportGenerator>);
    fn register_middleware(&mut self, position: MiddlewarePosition, layer: BoxedLayer);
    fn register_job(&mut self, job: JobDefinition);
    fn register_scheduled_job(&mut self, schedule: CronSchedule, job: JobDefinition);
    fn register_hashtag_source(&mut self, source: Box<dyn HashtagSource>);
    fn register_search_filter(&mut self, name: &str, filter: Box<dyn SearchFilter>);
    fn register_presence_channel(&mut self, prefix: &str, handler: Box<dyn PresenceHandler>);
    fn register_directory_column(&mut self, column: DirectoryColumnDefinition);
    fn register_api_key_scope(&mut self, resource: &str, action: &str, matcher: ApiScopeMatcher);
    fn register_rate_limiter(&mut self, limiter: RateLimiterDefinition);
    fn register_modifier(&mut self, name: &str, modifier: Box<dyn Modifier>);
    fn register_model_callback(&mut self, model: ModelTarget, callback_type: CallbackType, callback: Box<dyn ModelCallback>);
    fn register_validator(&mut self, model: ModelTarget, validator: Box<dyn Validator>);
}
