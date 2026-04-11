use serde_json::{json, Value};
use stevessr_services::categories::CategoryRecord;

/// Serialize a category record into the Discourse-compatible JSON format.
pub fn serialize_category(category: &CategoryRecord) -> Value {
    json!({
        "id": category.id,
        "name": category.name,
        "color": category.color,
        "text_color": category.text_color,
        "slug": category.slug,
        "topic_count": category.topic_count,
        "post_count": category.post_count,
        "position": category.position,
        "description": category.description,
        "description_text": category.description_text,
        "description_excerpt": category.description_excerpt,
        "topic_url": category.topic_url,
        "read_restricted": category.read_restricted,
        "permission": category.permission,
        "notification_level": category.notification_level,
        "can_edit": category.can_edit,
        "topic_template": category.topic_template,
        "has_children": category.has_children,
        "sort_order": category.sort_order,
        "sort_ascending": category.sort_ascending,
        "show_subcategory_list": category.show_subcategory_list,
        "num_featured_topics": category.num_featured_topics,
        "default_view": category.default_view,
        "subcategory_list_style": category.subcategory_list_style,
        "default_top_period": category.default_top_period,
        "default_list_filter": category.default_list_filter,
        "minimum_required_tags": category.minimum_required_tags,
        "navigate_to_first_post_after_read": category.navigate_to_first_post_after_read,
        "custom_fields": category.custom_fields,
        "allowed_tags": category.allowed_tags,
        "allowed_tag_groups": category.allowed_tag_groups,
        "allow_global_tags": category.allow_global_tags,
        "required_tag_groups": category.required_tag_groups,
        "parent_category_id": category.parent_category_id,
        "uploaded_logo": category.uploaded_logo,
        "uploaded_logo_dark": category.uploaded_logo_dark,
        "uploaded_background": category.uploaded_background,
        "subcategory_ids": category.subcategory_ids,
    })
}

/// Serialize a compact category for search results and side-references.
pub fn serialize_category_compact(category: &CategoryRecord) -> Value {
    json!({
        "id": category.id,
        "name": category.name,
        "color": category.color,
        "text_color": category.text_color,
        "slug": category.slug,
        "parent_category_id": category.parent_category_id,
        "read_restricted": category.read_restricted,
    })
}
