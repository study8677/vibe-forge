use sqlx::PgPool;
use stevessr_core::error::Result;

#[derive(Debug)]
pub struct SearchResult {
    pub post_id: i64,
    pub topic_id: i64,
    pub user_id: i64,
    pub headline: String,
    pub rank: f32,
}

#[derive(Debug)]
pub struct UserSearchResult {
    pub user_id: i64,
    pub username: String,
    pub name: Option<String>,
    pub rank: f32,
}

pub struct SearchParams {
    pub query: String,
    pub category_id: Option<i64>,
    pub tag_names: Vec<String>,
    pub user_id: Option<i64>,
    pub order: SearchOrder,
    pub page: i64,
    pub per_page: i64,
}

#[derive(Debug, Default)]
pub enum SearchOrder {
    #[default]
    Relevance,
    Latest,
    Likes,
    Views,
}

pub struct Searcher;

impl Searcher {
    pub async fn search_posts(pool: &PgPool, params: &SearchParams) -> Result<Vec<SearchResult>> {
        let offset = (params.page - 1) * params.per_page;

        let order_clause = match params.order {
            SearchOrder::Relevance => "rank DESC",
            SearchOrder::Latest => "p.created_at DESC",
            SearchOrder::Likes => "p.like_count DESC",
            SearchOrder::Views => "t.views DESC",
        };

        let mut query = format!(
            "SELECT p.id as post_id, p.topic_id, p.user_id,
                    ts_headline('english', psd.raw_data, websearch_to_tsquery('english', $1), 'MaxFragments=2, MaxWords=30') as headline,
                    ts_rank(psd.search_data, websearch_to_tsquery('english', $1)) as rank
             FROM post_search_data psd
             JOIN posts p ON p.id = psd.post_id
             JOIN topics t ON t.id = p.topic_id
             WHERE psd.search_data @@ websearch_to_tsquery('english', $1)
               AND p.deleted_at IS NULL
               AND t.deleted_at IS NULL
               AND t.visible = TRUE"
        );

        let mut param_idx = 2;

        if params.category_id.is_some() {
            query.push_str(&format!(" AND t.category_id = ${}", param_idx));
            param_idx += 1;
        }

        if params.user_id.is_some() {
            query.push_str(&format!(" AND p.user_id = ${}", param_idx));
            param_idx += 1;
        }

        query.push_str(&format!(" ORDER BY {} LIMIT ${} OFFSET ${}", order_clause, param_idx, param_idx + 1));

        let mut sqlx_query = sqlx::query_as::<_, (i64, i64, i64, String, f32)>(&query)
            .bind(&params.query);

        if let Some(cat_id) = params.category_id {
            sqlx_query = sqlx_query.bind(cat_id);
        }

        if let Some(uid) = params.user_id {
            sqlx_query = sqlx_query.bind(uid);
        }

        sqlx_query = sqlx_query.bind(params.per_page).bind(offset);

        let rows = sqlx_query.fetch_all(pool).await?;

        Ok(rows
            .into_iter()
            .map(|(post_id, topic_id, user_id, headline, rank)| SearchResult {
                post_id,
                topic_id,
                user_id,
                headline,
                rank,
            })
            .collect())
    }

    pub async fn search_users(pool: &PgPool, query: &str, limit: i64) -> Result<Vec<UserSearchResult>> {
        let rows: Vec<(i64, String, Option<String>, f32)> = sqlx::query_as(
            "SELECT u.id, u.username, u.name,
                    ts_rank(usd.search_data, websearch_to_tsquery('english', $1)) as rank
             FROM user_search_data usd
             JOIN users u ON u.id = usd.user_id
             WHERE usd.search_data @@ websearch_to_tsquery('english', $1)
               AND u.active = TRUE
             ORDER BY rank DESC
             LIMIT $2"
        )
        .bind(query)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(user_id, username, name, rank)| UserSearchResult {
                user_id,
                username,
                name,
                rank,
            })
            .collect())
    }
}
