use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use stevessr_core::traits::guardian::CurrentUser;

/// The outcome of a "before" hook.
#[derive(Debug)]
pub enum HookOutcome<T> {
    /// Continue with the (possibly modified) value.
    Continue(T),
    /// Abort the operation and return this error to the caller.
    Halt(crate::PluginError),
    /// Skip remaining hooks and proceed with this value.
    ShortCircuit(T),
}

/// Context available to hooks during execution.
pub struct HookContext {
    pub current_user: Option<CurrentUser>,
    pub request_id: Uuid,
    pub plugin_name: String,
}

/// A before-hook receives the input and may modify or reject it.
#[async_trait]
pub trait BeforeHook<I: Send + Sync>: Send + Sync {
    async fn execute(&self, input: I, ctx: &HookContext) -> HookOutcome<I>;
    fn priority(&self) -> i32 { 0 }
}

/// An after-hook receives the completed result and may observe or modify it.
#[async_trait]
pub trait AfterHook<I: Send + Sync, O: Send + Sync>: Send + Sync {
    async fn execute(&self, input: &I, output: O, ctx: &HookContext) -> O;
    fn priority(&self) -> i32 { 0 }
}

/// Identifies a hook registration for later removal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HookRegistrationId(pub u64);

/// Complete enumeration of all hook points in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HookPoint {
    // User hooks
    BeforeCreateUser, AfterCreateUser,
    BeforeUpdateUser, AfterUpdateUser,
    BeforeDestroyUser, AfterDestroyUser,
    BeforeSuspendUser, AfterSuspendUser,
    BeforeSilenceUser, AfterSilenceUser,
    BeforeUnsuspendUser, AfterUnsuspendUser,
    BeforeUnsilenceUser, AfterUnsilenceUser,
    BeforeActivateUser, AfterActivateUser,
    BeforeDeactivateUser, AfterDeactivateUser,
    BeforeAnonymizeUser, AfterAnonymizeUser,
    BeforeGrantTrustLevel, AfterGrantTrustLevel,
    BeforeGrantAdmin, AfterGrantAdmin,
    BeforeRevokeAdmin, AfterRevokeAdmin,
    BeforeGrantModeration, AfterGrantModeration,
    BeforeRevokeModeration, AfterRevokeModeration,
    BeforeChangeUsername, AfterChangeUsername,
    BeforeUpdateEmail, AfterUpdateEmail,
    BeforeApproveUser, AfterApproveUser,
    BeforeMergeUsers, AfterMergeUsers,

    // Topic hooks
    BeforeCreateTopic, AfterCreateTopic,
    BeforeUpdateTopic, AfterUpdateTopic,
    BeforeDestroyTopic, AfterDestroyTopic,
    BeforeRecoverTopic, AfterRecoverTopic,
    BeforeCloseTopic, AfterCloseTopic,
    BeforeOpenTopic, AfterOpenTopic,
    BeforeArchiveTopic, AfterArchiveTopic,
    BeforeUnarchiveTopic, AfterUnarchiveTopic,
    BeforePinTopic, AfterPinTopic,
    BeforeUnpinTopic, AfterUnpinTopic,
    BeforeMovePosts, AfterMovePosts,
    BeforeMergeTopics, AfterMergeTopics,
    BeforeChangeTopicCategory, AfterChangeTopicCategory,
    BeforeChangeTopicTags, AfterChangeTopicTags,
    BeforePublishTopic, AfterPublishTopic,

    // Post hooks
    BeforeCreatePost, AfterCreatePost,
    BeforeUpdatePost, AfterUpdatePost,
    BeforeDestroyPost, AfterDestroyPost,
    BeforeRecoverPost, AfterRecoverPost,
    BeforeLikePost, AfterLikePost,
    BeforeUnlikePost, AfterUnlikePost,
    BeforeFlagPost, AfterFlagPost,
    BeforeBookmarkPost, AfterBookmarkPost,
    BeforeLockPost, AfterLockPost,
    BeforeWikiPost, AfterWikiPost,
    BeforeHidePost, AfterHidePost,
    BeforeUnhidePost, AfterUnhidePost,

    // Category hooks
    BeforeCreateCategory, AfterCreateCategory,
    BeforeUpdateCategory, AfterUpdateCategory,
    BeforeDestroyCategory, AfterDestroyCategory,
    BeforeChangeCategoryPermissions, AfterChangeCategoryPermissions,

    // Group hooks
    BeforeCreateGroup, AfterCreateGroup,
    BeforeUpdateGroup, AfterUpdateGroup,
    BeforeDestroyGroup, AfterDestroyGroup,
    BeforeAddGroupMember, AfterAddGroupMember,
    BeforeRemoveGroupMember, AfterRemoveGroupMember,
    BeforeAddGroupOwner, AfterAddGroupOwner,

    // Tag hooks
    BeforeCreateTag, AfterCreateTag,
    BeforeUpdateTag, AfterUpdateTag,
    BeforeDestroyTag, AfterDestroyTag,

    // Notification hooks
    BeforeCreateNotification, AfterCreateNotification,
    BeforeSendEmailNotification, AfterSendEmailNotification,
    BeforeSendPushNotification, AfterSendPushNotification,

    // Badge hooks
    BeforeGrantBadge, AfterGrantBadge,
    BeforeRevokeBadge, AfterRevokeBadge,

    // Moderation hooks
    BeforeCreateReviewable, AfterCreateReviewable,
    BeforePerformReviewableAction, AfterPerformReviewableAction,

    // Invite hooks
    BeforeCreateInvite, AfterCreateInvite,
    BeforeRedeemInvite, AfterRedeemInvite,

    // Upload hooks
    BeforeCreateUpload, AfterCreateUpload,
    BeforeDestroyUpload, AfterDestroyUpload,

    // Auth hooks
    BeforeLogin, AfterLogin,
    BeforeLogout, AfterLogout,
    AfterOAuthAuthenticate,
    AfterSSOLogin,

    // Chat hooks
    BeforeCreateChatMessage, AfterCreateChatMessage,
    BeforeUpdateChatMessage, AfterUpdateChatMessage,
    BeforeDeleteChatMessage, AfterDeleteChatMessage,
    BeforeCreateChatChannel, AfterCreateChatChannel,

    // Webhook hooks
    BeforeEmitWebhook, AfterEmitWebhook,

    // Bookmark hooks
    BeforeCreateBookmark, AfterCreateBookmark,
    BeforeDestroyBookmark, AfterDestroyBookmark,

    // Search hooks
    BeforeSearch, AfterSearch,

    // Markdown/Content pipeline hooks
    BeforeCookMarkdown, AfterCookMarkdown,
    BeforeRenderOnebox, AfterRenderOnebox,

    // Site settings hooks
    AfterSiteSettingChanged,

    // Backup hooks
    BeforeCreateBackup, AfterCreateBackup,
    BeforeRestoreBackup, AfterRestoreBackup,

    // Email hooks
    BeforeSendEmail, AfterSendEmail,
    AfterReceiveEmail,

    // Poll hooks
    BeforeCastPollVote, AfterCastPollVote,

    // Draft hooks
    AfterSaveDraft, AfterDestroyDraft,
}

/// Typed hook parameter containers for type-safe hook dispatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserParams {
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub password: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopicParams {
    pub title: String,
    pub raw: String,
    pub category_id: Option<i64>,
    pub tags: Vec<String>,
    pub archetype: String,
    pub target_usernames: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostParams {
    pub topic_id: i64,
    pub raw: String,
    pub reply_to_post_number: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    pub query: String,
    pub page: Option<u32>,
    pub category_id: Option<i64>,
    pub tag: Option<String>,
    pub search_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookMarkdownParams {
    pub raw: String,
    pub topic_id: Option<i64>,
    pub user_id: Option<i64>,
}

/// Generic hook data for dynamic dispatch (used by WASM plugins).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericHookData {
    pub hook_point: HookPoint,
    pub payload: serde_json::Value,
}

/// Hook registry trait for registering hooks.
pub trait HookRegistry: Send + Sync {
    fn register_before_hook(
        &self,
        point: HookPoint,
        plugin_name: &str,
        handler: Arc<dyn BeforeHook<serde_json::Value>>,
    ) -> HookRegistrationId;

    fn register_after_hook(
        &self,
        point: HookPoint,
        plugin_name: &str,
        handler: Arc<dyn AfterHook<serde_json::Value, serde_json::Value>>,
    ) -> HookRegistrationId;

    fn unregister_hook(&self, id: HookRegistrationId);
}
