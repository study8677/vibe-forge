use serde_json::{json, Value};
use stevessr_services::users::UserRecord;

/// Serialize a user record into the Discourse-compatible JSON format.
///
/// When `include_private` is true (own profile or admin view),
/// private fields like email, second_factor_enabled, etc. are included.
pub fn serialize_user(user: &UserRecord, include_private: bool) -> Value {
    let mut data = json!({
        "id": user.id,
        "username": user.username,
        "name": user.name,
        "avatar_template": user.avatar_template,
        "title": user.title,
        "badge_count": user.badge_count,
        "trust_level": user.trust_level,
        "admin": user.admin,
        "moderator": user.moderator,
        "created_at": user.created_at,
        "last_seen_at": user.last_seen_at,
        "last_posted_at": user.last_posted_at,
        "profile_view_count": user.profile_view_count,
        "bio_raw": user.bio_raw,
        "bio_cooked": user.bio_cooked,
        "bio_excerpt": user.bio_excerpt,
        "website": user.website,
        "website_name": user.website_name,
        "location": user.location,
        "date_of_birth": user.date_of_birth,
        "can_edit": include_private,
        "can_edit_username": include_private,
        "can_edit_email": include_private,
        "can_edit_name": include_private,
        "can_send_private_messages": user.trust_level >= 1,
        "can_send_private_message_to_user": true,
        "featured_topic": user.featured_topic,
        "invited_by": user.invited_by,
        "custom_fields": user.custom_fields,
        "user_fields": user.user_fields,
        "primary_group_id": user.primary_group_id,
        "primary_group_name": user.primary_group_name,
        "primary_group_flair_url": user.primary_group_flair_url,
        "primary_group_flair_bg_color": user.primary_group_flair_bg_color,
        "primary_group_flair_color": user.primary_group_flair_color,
        "flair_name": user.flair_name,
        "flair_url": user.flair_url,
        "flair_bg_color": user.flair_bg_color,
        "flair_color": user.flair_color,
        "card_background_upload_url": user.card_background_upload_url,
        "profile_background_upload_url": user.profile_background_upload_url,
        "groups": user.groups,
        "user_option": {
            "theme_ids": user.theme_ids,
            "text_size": user.text_size,
            "email_level": user.email_level,
            "email_messages_level": user.email_messages_level,
            "notification_level_when_replying": user.notification_level_when_replying,
        },
        "featured_user_badge_ids": user.featured_user_badge_ids,
    });

    if include_private {
        if let Some(obj) = data.as_object_mut() {
            obj.insert("email".to_string(), json!(user.email));
            obj.insert("secondary_emails".to_string(), json!(user.secondary_emails));
            obj.insert("unconfirmed_emails".to_string(), json!(user.unconfirmed_emails));
            obj.insert("unread_notifications".to_string(), json!(user.unread_notifications));
            obj.insert(
                "unread_high_priority_notifications".to_string(),
                json!(user.unread_high_priority_notifications),
            );
            obj.insert("read_first_notification".to_string(), json!(user.read_first_notification));
            obj.insert("second_factor_enabled".to_string(), json!(user.second_factor_enabled));
            obj.insert("associated_accounts".to_string(), json!(user.associated_accounts));
            obj.insert("muted_users".to_string(), json!(user.muted_users));
            obj.insert("ignored_users".to_string(), json!(user.ignored_users));
            obj.insert("muted_categories".to_string(), json!(user.muted_categories));
            obj.insert("watched_categories".to_string(), json!(user.watched_categories));
            obj.insert("tracked_categories".to_string(), json!(user.tracked_categories));
            obj.insert("muted_tags".to_string(), json!(user.muted_tags));
            obj.insert("watched_tags".to_string(), json!(user.watched_tags));
            obj.insert("tracked_tags".to_string(), json!(user.tracked_tags));
        }
    }

    data
}

/// Serialize a compact user representation for listings (topic posters, search results, etc.)
pub fn serialize_user_compact(user: &UserRecord) -> Value {
    json!({
        "id": user.id,
        "username": user.username,
        "name": user.name,
        "avatar_template": user.avatar_template,
        "admin": user.admin,
        "moderator": user.moderator,
        "trust_level": user.trust_level,
    })
}
