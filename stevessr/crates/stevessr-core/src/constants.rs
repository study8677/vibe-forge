/// System-wide constants mirroring Discourse's SiteSetting defaults and system constants.

// Username constraints
pub const USERNAME_MIN_LENGTH: usize = 3;
pub const USERNAME_MAX_LENGTH: usize = 20;
pub const USERNAME_PATTERN: &str = r"^[a-zA-Z0-9_.-]+$";

// Title constraints
pub const TOPIC_TITLE_MIN_LENGTH: usize = 15;
pub const TOPIC_TITLE_MAX_LENGTH: usize = 255;

// Post constraints
pub const POST_MIN_LENGTH: usize = 20;
pub const POST_MAX_LENGTH: usize = 32000;
pub const MAX_POST_MENTIONS: usize = 10;

// Category constraints
pub const CATEGORY_NAME_MAX_LENGTH: usize = 50;
pub const MAX_CATEGORY_NESTING: usize = 1;

// Tag constraints
pub const TAG_NAME_MAX_LENGTH: usize = 100;
pub const MAX_TAGS_PER_TOPIC: usize = 5;

// Trust level thresholds (defaults, overrideable via site settings)
pub const TL1_REQUIRES_TOPICS_ENTERED: i32 = 5;
pub const TL1_REQUIRES_POSTS_READ: i32 = 30;
pub const TL1_REQUIRES_TIME_SPENT_MINS: i32 = 10;
pub const TL2_REQUIRES_TOPICS_ENTERED: i32 = 20;
pub const TL2_REQUIRES_POSTS_READ: i32 = 100;
pub const TL2_REQUIRES_TIME_SPENT_MINS: i32 = 60;
pub const TL2_REQUIRES_DAYS_VISITED: i32 = 15;
pub const TL2_REQUIRES_LIKES_GIVEN: i32 = 1;
pub const TL2_REQUIRES_LIKES_RECEIVED: i32 = 1;
pub const TL3_REQUIRES_DAYS_VISITED: i32 = 50;
pub const TL3_REQUIRES_TOPICS_REPLIED_TO: i32 = 10;
pub const TL3_REQUIRES_TOPICS_VIEWED: i32 = 25;
pub const TL3_REQUIRES_POSTS_READ: i32 = 25;
pub const TL3_REQUIRES_LIKES_GIVEN: i32 = 30;
pub const TL3_REQUIRES_LIKES_RECEIVED: i32 = 20;

// Rate limits
pub const MAX_TOPICS_PER_DAY: usize = 20;
pub const MAX_POSTS_PER_DAY: usize = 100;
pub const MAX_LIKES_PER_DAY: usize = 50;
pub const MAX_FLAGS_PER_DAY: usize = 10;
pub const MAX_BOOKMARKS_PER_DAY: usize = 20;
pub const MAX_EDITS_PER_DAY: usize = 30;
pub const MAX_INVITES_PER_DAY: usize = 10;

// System user ID
pub const SYSTEM_USER_ID: i64 = -1;

// Default group IDs (automatic groups, mirroring Discourse)
pub const EVERYONE_GROUP_ID: i64 = 0;
pub const ADMINS_GROUP_ID: i64 = 1;
pub const MODERATORS_GROUP_ID: i64 = 2;
pub const STAFF_GROUP_ID: i64 = 3;
pub const TL0_GROUP_ID: i64 = 10;
pub const TL1_GROUP_ID: i64 = 11;
pub const TL2_GROUP_ID: i64 = 12;
pub const TL3_GROUP_ID: i64 = 13;
pub const TL4_GROUP_ID: i64 = 14;

// Digest
pub const DEFAULT_DIGEST_MINUTES: i32 = 10080; // 7 days

// Notifications
pub const MAX_NOTIFICATIONS_PER_PAGE: usize = 60;

// Search
pub const MAX_SEARCH_RESULTS: usize = 50;

// Chat
pub const MAX_CHAT_MESSAGE_LENGTH: usize = 6000;
pub const CHAT_MAX_DIRECT_MESSAGE_USERS: usize = 20;

// Uploads
pub const MAX_UPLOAD_NAME_LENGTH: usize = 255;
