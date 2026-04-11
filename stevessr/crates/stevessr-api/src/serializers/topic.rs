use serde_json::{json, Value};
use stevessr_services::topics::TopicRecord;

/// Serialize a topic record into the Discourse-compatible JSON format
/// for the "basic_topic" representation used in update responses and listings.
pub fn serialize_topic(topic: &TopicRecord) -> Value {
    json!({
        "id": topic.id,
        "title": topic.title,
        "fancy_title": topic.fancy_title,
        "slug": topic.slug,
        "posts_count": topic.posts_count,
        "reply_count": topic.reply_count,
        "highest_post_number": topic.highest_post_number,
        "image_url": topic.image_url,
        "created_at": topic.created_at,
        "last_posted_at": topic.last_posted_at,
        "bumped": topic.bumped,
        "bumped_at": topic.bumped_at,
        "archetype": topic.archetype,
        "unseen": topic.unseen,
        "pinned": topic.pinned,
        "unpinned": topic.unpinned,
        "visible": topic.visible,
        "closed": topic.closed,
        "archived": topic.archived,
        "bookmarked": topic.bookmarked,
        "liked": topic.liked,
        "views": topic.views,
        "like_count": topic.like_count,
        "has_summary": topic.has_summary,
        "last_poster_username": topic.last_poster_username,
        "category_id": topic.category_id,
        "pinned_globally": topic.pinned_globally,
        "featured_link": topic.featured_link,
        "has_accepted_answer": topic.has_accepted_answer,
        "tags": topic.tags,
        "tags_descriptions": topic.tags_descriptions,
        "posters": topic.posters,
        "word_count": topic.word_count,
    })
}

/// Serialize a topic for the topic list (category pages, latest, etc.)
/// This is a lighter representation than the full topic view.
pub fn serialize_topic_list_item(topic: &TopicRecord) -> Value {
    json!({
        "id": topic.id,
        "title": topic.title,
        "fancy_title": topic.fancy_title,
        "slug": topic.slug,
        "posts_count": topic.posts_count,
        "reply_count": topic.reply_count,
        "highest_post_number": topic.highest_post_number,
        "created_at": topic.created_at,
        "last_posted_at": topic.last_posted_at,
        "bumped_at": topic.bumped_at,
        "unseen": topic.unseen,
        "pinned": topic.pinned,
        "visible": topic.visible,
        "closed": topic.closed,
        "archived": topic.archived,
        "views": topic.views,
        "like_count": topic.like_count,
        "category_id": topic.category_id,
        "pinned_globally": topic.pinned_globally,
        "tags": topic.tags,
        "posters": topic.posters,
    })
}
