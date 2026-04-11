use serde::{Deserialize, Serialize};

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
        #[sqlx(transparent)]
        pub struct $name(pub i64);

        impl $name {
            pub fn new(id: i64) -> Self {
                Self(id)
            }

            pub fn into_inner(self) -> i64 {
                self.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<i64> for $name {
            fn from(id: i64) -> Self {
                Self(id)
            }
        }

        impl From<$name> for i64 {
            fn from(id: $name) -> Self {
                id.0
            }
        }
    };
}

define_id!(UserId);
define_id!(TopicId);
define_id!(PostId);
define_id!(CategoryId);
define_id!(GroupId);
define_id!(TagId);
define_id!(TagGroupId);
define_id!(BadgeId);
define_id!(BadgeTypeId);
define_id!(BadgeGroupingId);
define_id!(NotificationId);
define_id!(BookmarkId);
define_id!(DraftId);
define_id!(InviteId);
define_id!(UploadId);
define_id!(ReviewableId);
define_id!(WebHookId);
define_id!(ApiKeyId);
define_id!(UserApiKeyId);
define_id!(ChatChannelId);
define_id!(ChatMessageId);
define_id!(ChatThreadId);
define_id!(PollId);
define_id!(PollOptionId);
define_id!(SidebarSectionId);
define_id!(ThemeId);
define_id!(ColorSchemeId);
define_id!(FormTemplateId);
define_id!(PermalinkId);
define_id!(EmailLogId);
define_id!(SearchLogId);
define_id!(BackupId);
define_id!(PluginStoreRowId);
