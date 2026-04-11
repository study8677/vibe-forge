use serde::{Deserialize, Serialize};

/// Marker trait for all domain events.
pub trait DomainEvent: Send + Sync + Serialize + for<'de> Deserialize<'de> + 'static {
    fn event_name() -> &'static str;
}

macro_rules! define_event {
    ($name:ident { $($field:ident : $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl DomainEvent for $name {
            fn event_name() -> &'static str {
                stringify!($name)
            }
        }
    };
}

// Topic events
define_event!(TopicCreated { topic_id: i64, user_id: i64, category_id: Option<i64> });
define_event!(TopicEdited { topic_id: i64, editor_id: i64 });
define_event!(TopicDestroyed { topic_id: i64, deleted_by_id: i64 });
define_event!(TopicRecovered { topic_id: i64, recovered_by_id: i64 });
define_event!(TopicClosed { topic_id: i64, closed_by_id: i64 });
define_event!(TopicOpened { topic_id: i64, opened_by_id: i64 });
define_event!(TopicArchived { topic_id: i64 });
define_event!(TopicPinned { topic_id: i64, globally: bool });
define_event!(TopicMoved { topic_id: i64, from_category_id: i64, to_category_id: i64 });
define_event!(TopicTagsChanged { topic_id: i64, added: Vec<i64>, removed: Vec<i64> });

// Post events
define_event!(PostCreated { post_id: i64, topic_id: i64, user_id: i64, post_number: i32 });
define_event!(PostEdited { post_id: i64, editor_id: i64, revision_number: i32 });
define_event!(PostDestroyed { post_id: i64 });
define_event!(PostRecovered { post_id: i64 });
define_event!(PostLiked { post_id: i64, user_id: i64 });
define_event!(PostFlagged { post_id: i64, user_id: i64, flag_type: i32 });
define_event!(PostHidden { post_id: i64, reason: i32 });

// User events
define_event!(UserCreated { user_id: i64 });
define_event!(UserActivated { user_id: i64 });
define_event!(UserApproved { user_id: i64, approved_by_id: i64 });
define_event!(UserSuspended { user_id: i64, suspended_by_id: i64, reason: String });
define_event!(UserUnsuspended { user_id: i64 });
define_event!(UserSilenced { user_id: i64 });
define_event!(UserUnsilenced { user_id: i64 });
define_event!(UserLoggedIn { user_id: i64, ip: String });
define_event!(UserLoggedOut { user_id: i64 });
define_event!(UserTrustLevelChanged { user_id: i64, old_level: i32, new_level: i32 });
define_event!(UserEmailConfirmed { user_id: i64, email: String });
define_event!(UserDestroyed { user_id: i64 });
define_event!(UserUpdated { user_id: i64 });
define_event!(UserAnonymized { user_id: i64 });

// Category events
define_event!(CategoryCreated { category_id: i64 });
define_event!(CategoryUpdated { category_id: i64 });
define_event!(CategoryDestroyed { category_id: i64 });

// Group events
define_event!(GroupCreated { group_id: i64 });
define_event!(GroupUpdated { group_id: i64 });
define_event!(GroupDestroyed { group_id: i64 });
define_event!(GroupMemberAdded { group_id: i64, user_id: i64 });
define_event!(GroupMemberRemoved { group_id: i64, user_id: i64 });

// Tag events
define_event!(TagCreated { tag_id: i64 });
define_event!(TagUpdated { tag_id: i64 });
define_event!(TagDestroyed { tag_id: i64 });

// Notification events
define_event!(NotificationCreated { notification_id: i64, user_id: i64, notification_type: i32 });

// Badge events
define_event!(BadgeGranted { user_id: i64, badge_id: i64 });
define_event!(BadgeRevoked { user_id: i64, badge_id: i64 });

// Moderation events
define_event!(ReviewableCreated { reviewable_id: i64, reviewable_type: String });
define_event!(ReviewableStateChanged { reviewable_id: i64, old_status: i32, new_status: i32 });

// Invite events
define_event!(InviteCreated { invite_id: i64, invited_by_id: i64 });
define_event!(InviteRedeemed { invite_id: i64, user_id: i64 });

// Chat events
define_event!(ChatMessageCreated { message_id: i64, channel_id: i64, user_id: i64 });
define_event!(ChatMessageEdited { message_id: i64 });
define_event!(ChatMessageDeleted { message_id: i64 });
define_event!(ChatChannelCreated { channel_id: i64 });

// Settings events
define_event!(SiteSettingChanged { name: String, old_value: String, new_value: String });

// Bookmark events
define_event!(BookmarkCreated { bookmark_id: i64, user_id: i64 });
define_event!(BookmarkDestroyed { bookmark_id: i64 });

// Poll events
define_event!(PollVoteCast { poll_id: i64, user_id: i64 });

// Email events
define_event!(EmailSent { to: String, email_type: String });
define_event!(EmailReceived { incoming_email_id: i64 });

// Backup events
define_event!(BackupCreated { filename: String });
define_event!(BackupRestored { filename: String });
