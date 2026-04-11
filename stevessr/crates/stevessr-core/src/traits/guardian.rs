use crate::types::ids::*;
use crate::types::trust_level::TrustLevel;

/// Permission checking trait. The Guardian determines what a user can do.
/// Mirrors Discourse's Guardian module.
pub struct CurrentUser {
    pub id: UserId,
    pub username: String,
    pub trust_level: TrustLevel,
    pub admin: bool,
    pub moderator: bool,
    pub silenced_till: Option<chrono::DateTime<chrono::Utc>>,
    pub suspended_till: Option<chrono::DateTime<chrono::Utc>>,
    pub groups: Vec<GroupId>,
}

impl CurrentUser {
    pub fn is_staff(&self) -> bool {
        self.admin || self.moderator
    }

    pub fn is_admin(&self) -> bool {
        self.admin
    }

    pub fn is_moderator(&self) -> bool {
        self.moderator
    }

    pub fn is_silenced(&self) -> bool {
        self.silenced_till
            .map(|t| t > chrono::Utc::now())
            .unwrap_or(false)
    }

    pub fn is_suspended(&self) -> bool {
        self.suspended_till
            .map(|t| t > chrono::Utc::now())
            .unwrap_or(false)
    }

    pub fn is_in_group(&self, group_id: GroupId) -> bool {
        self.groups.contains(&group_id)
    }
}

/// Guardian provides permission checks.
pub trait Guardian {
    fn current_user(&self) -> Option<&CurrentUser>;

    fn is_authenticated(&self) -> bool {
        self.current_user().is_some()
    }

    fn is_admin(&self) -> bool {
        self.current_user().map(|u| u.is_admin()).unwrap_or(false)
    }

    fn is_staff(&self) -> bool {
        self.current_user().map(|u| u.is_staff()).unwrap_or(false)
    }

    fn is_moderator(&self) -> bool {
        self.current_user().map(|u| u.is_moderator()).unwrap_or(false)
    }

    fn can_create_topic(&self, category_id: Option<CategoryId>) -> bool {
        let Some(user) = self.current_user() else { return false };
        if user.is_silenced() || user.is_suspended() { return false; }
        let _ = category_id; // TODO: check category permissions
        user.trust_level.can_create_topic()
    }

    fn can_create_post(&self, topic_id: TopicId) -> bool {
        let Some(user) = self.current_user() else { return false };
        if user.is_silenced() || user.is_suspended() { return false; }
        let _ = topic_id;
        user.trust_level.can_reply()
    }

    fn can_edit_post(&self, post_user_id: UserId, _post_id: PostId, wiki: bool) -> bool {
        let Some(user) = self.current_user() else { return false };
        if user.is_staff() { return true; }
        if wiki && user.trust_level.can_edit_wiki() { return true; }
        user.id == post_user_id
    }

    fn can_delete_post(&self, post_user_id: UserId, _post_id: PostId) -> bool {
        let Some(user) = self.current_user() else { return false };
        if user.is_staff() { return true; }
        user.id == post_user_id
    }

    fn can_like(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        !user.is_silenced() && !user.is_suspended()
    }

    fn can_flag(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        if user.is_silenced() || user.is_suspended() { return false; }
        user.trust_level.can_flag()
    }

    fn can_send_pm(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        if user.is_silenced() || user.is_suspended() { return false; }
        user.trust_level.can_send_pm() || user.is_staff()
    }

    fn can_invite(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        user.trust_level.can_invite() || user.is_staff()
    }

    fn can_tag_topics(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        user.trust_level.can_tag_topics() || user.is_staff()
    }

    fn can_upload(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        !user.is_suspended()
    }

    fn can_upload_images(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        user.trust_level.can_upload_images() || user.is_staff()
    }

    fn can_create_category(&self) -> bool {
        self.is_staff()
    }

    fn can_edit_category(&self) -> bool {
        self.is_staff()
    }

    fn can_delete_category(&self) -> bool {
        self.is_admin()
    }

    fn can_create_group(&self) -> bool {
        self.is_staff()
    }

    fn can_manage_group(&self, _group_id: GroupId) -> bool {
        self.is_staff() // TODO: check group ownership
    }

    fn can_create_tag(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        user.trust_level.can_tag_topics() || user.is_staff()
    }

    fn can_manage_tags(&self) -> bool {
        self.is_staff()
    }

    fn can_review(&self) -> bool {
        self.is_staff()
    }

    fn can_see_admin(&self) -> bool {
        self.is_admin()
    }

    fn can_manage_site_settings(&self) -> bool {
        self.is_admin()
    }

    fn can_manage_webhooks(&self) -> bool {
        self.is_admin()
    }

    fn can_manage_api_keys(&self) -> bool {
        self.is_admin()
    }

    fn can_manage_badges(&self) -> bool {
        self.is_admin()
    }

    fn can_manage_plugins(&self) -> bool {
        self.is_admin()
    }

    fn can_backup(&self) -> bool {
        self.is_admin()
    }

    fn can_create_chat_channel(&self) -> bool {
        self.is_staff()
    }

    fn can_send_chat_message(&self) -> bool {
        let Some(user) = self.current_user() else { return false };
        !user.is_silenced() && !user.is_suspended()
    }

    fn can_moderate_chat(&self) -> bool {
        self.is_staff()
    }

    fn can_suspend_user(&self) -> bool {
        self.is_staff()
    }

    fn can_silence_user(&self) -> bool {
        self.is_staff()
    }

    fn can_grant_admin(&self) -> bool {
        self.is_admin()
    }

    fn can_grant_moderation(&self) -> bool {
        self.is_admin()
    }

    fn can_impersonate(&self) -> bool {
        self.is_admin()
    }

    fn can_see_emails(&self) -> bool {
        self.is_admin()
    }

    fn can_see_ip_addresses(&self) -> bool {
        self.is_staff()
    }

    fn can_delete_user(&self, target_user_id: UserId) -> bool {
        let Some(user) = self.current_user() else { return false };
        user.is_admin() && user.id != target_user_id
    }

    fn can_merge_users(&self) -> bool {
        self.is_admin()
    }

    fn can_anonymize_user(&self) -> bool {
        self.is_admin()
    }
}
