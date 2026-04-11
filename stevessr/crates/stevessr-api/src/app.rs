use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use crate::state::AppState;
use crate::routes;

#[cfg(feature = "full-api")]
use axum::routing::{post, put, delete};

pub fn build_router(state: AppState) -> Router {
    let router = Router::new()
        // Health check
        .route("/srv/status", get(routes::health::status));

    #[cfg(feature = "full-api")]
    let router = router
        // All route registrations require the full-api feature
        .route("/placeholder", get(routes::health::status));

    #[cfg(not(feature = "full-api"))]
    let router = router;

    router
        // Middleware
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}

#[cfg(feature = "full-api")]
fn _build_full_router(state: AppState) -> Router {
    use axum::routing::{post, put, delete};

    Router::new()
        // Health check
        .route("/srv/status", get(routes::health::status))

        // Site bootstrap
        .route("/site.json", get(routes::site::show))
        .route("/about.json", get(routes::about::json))

        // Session / Auth
        .route("/session", post(routes::session::create))
        .route("/session/{id}", delete(routes::session::destroy))
        .route("/session/csrf", get(routes::session::csrf))
        .route("/session/current", get(routes::session::current))
        .route("/session/forgot_password", post(routes::session::forgot_password))

        // Users
        .route("/u", post(routes::users::create))
        .route("/u/{username}", get(routes::users::show))
        .route("/u/{username}", put(routes::users::update))
        .route("/u/{username}/summary", get(routes::users::summary))
        .route("/u/{username}/activity", get(routes::users::activity))
        .route("/u/{username}/badges", get(routes::users::badges))
        .route("/u/{username}/notifications", get(routes::users::notifications))
        .route("/u/{username}/bookmarks", get(routes::users::bookmarks))
        .route("/u/{username}/drafts", get(routes::users::drafts))
        .route("/u/{username}", delete(routes::users::destroy))
        .route("/u/search/users", get(routes::users::search))

        // Topics
        .route("/t", post(routes::topics::create))
        .route("/t/{id}", get(routes::topics::show))
        .route("/t/{id}", put(routes::topics::update))
        .route("/t/{id}", delete(routes::topics::destroy))
        .route("/t/{id}/status", put(routes::topics::update_status))
        .route("/t/{id}/timer", put(routes::topics::set_timer))
        .route("/t/{id}/invite", put(routes::topics::invite))
        .route("/t/{id}/move-posts", post(routes::topics::move_posts))
        .route("/t/{id}/merge-topic", post(routes::topics::merge))
        .route("/t/{id}/timings", post(routes::topics::timings))
        .route("/t/{slug}/{id}", get(routes::topics::show_with_slug))
        .route("/t/{slug}/{id}/{post_number}", get(routes::topics::show_post))

        // Posts
        .route("/posts", post(routes::posts::create))
        .route("/posts/{id}", get(routes::posts::show))
        .route("/posts/{id}", put(routes::posts::update))
        .route("/posts/{id}", delete(routes::posts::destroy))
        .route("/posts/{id}/revisions/{revision}", get(routes::posts::show_revision))
        .route("/posts/{id}/wiki", put(routes::posts::toggle_wiki))
        .route("/posts/{id}/locked", put(routes::posts::toggle_locked))
        .route("/posts/{id}/replies", get(routes::posts::replies))

        // Post Actions (likes, flags)
        .route("/post_actions", post(routes::post_actions::create))
        .route("/post_actions/{id}", delete(routes::post_actions::destroy))

        // Categories
        .route("/categories", get(routes::categories::index))
        .route("/categories", post(routes::categories::create))
        .route("/categories/{id}", put(routes::categories::update))
        .route("/categories/{id}", delete(routes::categories::destroy))
        .route("/categories/reorder", post(routes::categories::reorder))
        .route("/c/{slug}/{id}", get(routes::categories::show))
        .route("/c/{slug}/{id}/l/latest", get(routes::categories::topic_list))

        // Groups
        .route("/g", get(routes::groups::index))
        .route("/g", post(routes::groups::create))
        .route("/g/{name}", get(routes::groups::show))
        .route("/g/{id}", put(routes::groups::update))
        .route("/g/{id}", delete(routes::groups::destroy))
        .route("/g/{name}/members", get(routes::groups::members))
        .route("/g/{id}/members", put(routes::groups::add_members))
        .route("/g/{id}/members", delete(routes::groups::remove_members))

        // Tags
        .route("/tags", get(routes::tags::index))
        .route("/tag/{name}", get(routes::tags::show))
        .route("/tag/{id}", put(routes::tags::update))
        .route("/tag/{id}", delete(routes::tags::destroy))

        // Search
        .route("/search", get(routes::search::query))

        // Notifications
        .route("/notifications", get(routes::notifications::index))
        .route("/notifications/mark-read", put(routes::notifications::mark_read))

        // Bookmarks
        .route("/bookmarks", post(routes::bookmarks::create))
        .route("/bookmarks/{id}", put(routes::bookmarks::update))
        .route("/bookmarks/{id}", delete(routes::bookmarks::destroy))

        // Drafts
        .route("/drafts", get(routes::drafts::index))
        .route("/drafts", post(routes::drafts::create))
        .route("/drafts/{id}", delete(routes::drafts::destroy))

        // Uploads
        .route("/uploads", post(routes::uploads::create))

        // Badges
        .route("/badges", get(routes::badges::index))
        .route("/user_badges", post(routes::badges::grant))

        // Invites
        .route("/invites", post(routes::invites::create))
        .route("/invites/{id}", put(routes::invites::update))
        .route("/invites/{id}", delete(routes::invites::destroy))
        .route("/invites/show/{invite_key}", get(routes::invites::show))

        // Polls
        .route("/polls/vote", put(routes::polls::vote))
        .route("/polls/toggle_status", put(routes::polls::toggle_status))
        .route("/polls/voters", get(routes::polls::voters))

        // Directory
        .route("/directory_items", get(routes::directory::index))

        // Chat
        .route("/chat/channels", get(routes::chat::channels::index))
        .route("/chat/channels", post(routes::chat::channels::create))
        .route("/chat/channels/{id}", get(routes::chat::channels::show))
        .route("/chat/channels/{id}", put(routes::chat::channels::update))
        .route("/chat/channels/{id}/messages", get(routes::chat::messages::index))
        .route("/chat/channels/{id}/messages", post(routes::chat::messages::create))
        .route("/chat/channels/{id}/messages/{message_id}", put(routes::chat::messages::update))
        .route("/chat/channels/{id}/messages/{message_id}", delete(routes::chat::messages::destroy))
        .route("/chat/channels/{id}/threads", get(routes::chat::threads::index))

        // Review queue
        .route("/review", get(routes::review::index))
        .route("/review/{id}", get(routes::review::show))
        .route("/review/{id}/perform/{action}", put(routes::review::perform_action))

        // Admin
        .route("/admin/dashboard", get(routes::admin::dashboard::index))
        .route("/admin/site_settings", get(routes::admin::site_settings::index))
        .route("/admin/site_settings/{id}", put(routes::admin::site_settings::update))
        .route("/admin/users", get(routes::admin::users::index))
        .route("/admin/users/{id}/suspend", put(routes::admin::users::suspend))
        .route("/admin/users/{id}/unsuspend", put(routes::admin::users::unsuspend))
        .route("/admin/users/{id}/silence", put(routes::admin::users::silence))
        .route("/admin/users/{id}/grant_admin", put(routes::admin::users::grant_admin))
        .route("/admin/users/{id}/revoke_admin", put(routes::admin::users::revoke_admin))
        .route("/admin/badges", get(routes::admin::badges::index))
        .route("/admin/badges", post(routes::admin::badges::create))
        .route("/admin/backups", get(routes::admin::backups::index))
        .route("/admin/backups", post(routes::admin::backups::create))
        .route("/admin/plugins", get(routes::admin::plugins::index))
        .route("/admin/web_hooks", get(routes::admin::webhooks::index))
        .route("/admin/web_hooks", post(routes::admin::webhooks::create))
        .route("/admin/api", get(routes::admin::api_keys::index))
        .route("/admin/api", post(routes::admin::api_keys::create))
        .route("/admin/reports/{type}", get(routes::admin::reports::show))
        .route("/admin/logs", get(routes::admin::logs::index))

        // WebSocket
        .route("/ws", get(routes::websocket_handler::ws_upgrade))
        .route("/message-bus/poll", get(routes::websocket_handler::message_bus_poll))

        // Middleware
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
