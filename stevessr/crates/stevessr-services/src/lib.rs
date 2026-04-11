// Internal service modules (prefixed to avoid conflicts with facade re-exports)
pub mod user;
pub mod topic;
pub mod post;
pub mod category;
pub mod group;
pub mod tag;
pub mod notification;
pub mod badge;
#[path = "search/mod.rs"]
pub mod search_service;
pub mod moderation;
pub mod email;
pub mod upload;
pub mod auth;
pub mod chat;
pub mod invite;
pub mod bookmark;
pub mod draft;
pub mod poll;
pub mod webhook;
pub mod backup;
pub mod admin;
pub mod guardian;

// Facade modules providing flat function APIs for the API layer
pub mod facades;

pub use facades::users_facade as users;
pub use facades::topics_facade as topics;
pub use facades::posts_facade as posts;
pub use facades::categories_facade as categories;
pub use facades::groups_facade as groups;
pub use facades::tags_facade as tags;
pub use facades::badges_facade as badges;
pub use facades::notifications_facade as notifications;
pub use facades::bookmarks_facade as bookmarks;
pub use facades::drafts_facade as drafts;
pub use facades::polls_facade as polls;
pub use facades::uploads_facade as uploads;
pub use facades::invites_facade as invites;
pub use facades::session_facade as session;
pub use facades::site_facade as site;
pub use facades::directory_facade as directory;
pub use facades::review_facade as review;
pub use facades::post_actions_facade as post_actions;
pub use facades::pubsub_facade as pubsub;
pub use facades::search_facade as search;
