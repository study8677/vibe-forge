use sqlx::PgPool;
use stevessr_core::error::{Error, Result, ValidationErrors};
use stevessr_core::constants::*;
use stevessr_db::models::topic::Topic;
use stevessr_db::models::post::Post;

pub struct CreateTopicParams {
    pub title: String,
    pub raw: String,
    pub category_id: Option<i64>,
    pub tags: Vec<String>,
    pub user_id: i64,
    pub archetype: String,
    pub target_usernames: Vec<String>,
}

pub struct TopicCreator;

impl TopicCreator {
    pub async fn create(pool: &PgPool, params: CreateTopicParams) -> Result<(Topic, Post)> {
        Self::validate(&params)?;

        let slug = slug::slugify(&params.title);

        let topic = Topic::create(
            pool,
            &params.title,
            &slug,
            params.user_id,
            params.category_id,
            &params.archetype,
        ).await?;

        // Create the first post
        let post = Post::create(
            pool,
            params.user_id,
            topic.id,
            1, // post_number
            &params.raw,
            &params.raw, // cooked = raw for now, TODO: run through markdown pipeline
            None, // reply_to_post_number
            1,    // post_type: regular
        ).await?;

        // Add tags
        for tag_name in &params.tags {
            if let Ok(Some(tag)) = stevessr_db::models::tag::Tag::find_by_name(pool, tag_name).await {
                stevessr_db::models::topic_tag::TopicTag::create(pool, topic.id, tag.id).await?;
            }
        }

        // Handle private messages
        if params.archetype == "private_message" {
            for username in &params.target_usernames {
                if let Ok(Some(user)) = stevessr_db::models::user::User::find_by_username(pool, username).await {
                    stevessr_db::models::topic_allowed_user::TopicAllowedUser::create(pool, user.id, topic.id).await?;
                }
            }
            stevessr_db::models::topic_allowed_user::TopicAllowedUser::create(pool, params.user_id, topic.id).await?;
        }

        Ok((topic, post))
    }

    fn validate(params: &CreateTopicParams) -> Result<()> {
        let mut errors = ValidationErrors::new();

        if params.title.len() < TOPIC_TITLE_MIN_LENGTH {
            errors.add("title", format!("must be at least {} characters", TOPIC_TITLE_MIN_LENGTH));
        }
        if params.title.len() > TOPIC_TITLE_MAX_LENGTH {
            errors.add("title", format!("must be at most {} characters", TOPIC_TITLE_MAX_LENGTH));
        }
        if params.raw.len() < POST_MIN_LENGTH {
            errors.add("raw", format!("must be at least {} characters", POST_MIN_LENGTH));
        }
        if params.tags.len() > MAX_TAGS_PER_TOPIC {
            errors.add("tags", format!("at most {} tags allowed", MAX_TAGS_PER_TOPIC));
        }

        if errors.is_empty() { Ok(()) } else { Err(Error::Validation(errors)) }
    }
}
