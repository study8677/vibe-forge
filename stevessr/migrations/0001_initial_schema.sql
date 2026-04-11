-- =============================================================================
-- stevessr: 0001_initial_schema.sql
-- Comprehensive Discourse-equivalent schema for PostgreSQL
-- All tables, indexes, foreign keys, and unique constraints
-- =============================================================================

BEGIN;

-- =============================================================================
-- 1. USERS SUBSYSTEM
-- =============================================================================

CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    username        VARCHAR(60) NOT NULL UNIQUE,
    username_lower  VARCHAR(60) NOT NULL UNIQUE,
    name            VARCHAR(255),
    active          BOOLEAN NOT NULL DEFAULT FALSE,
    approved        BOOLEAN NOT NULL DEFAULT FALSE,
    approved_by_id  BIGINT,
    approved_at     TIMESTAMPTZ,
    admin           BOOLEAN NOT NULL DEFAULT FALSE,
    moderator       BOOLEAN NOT NULL DEFAULT FALSE,
    trust_level     SMALLINT NOT NULL DEFAULT 0,
    staged          BOOLEAN NOT NULL DEFAULT FALSE,
    date_of_birth   DATE,
    ip_address      INET,
    registration_ip_address INET,
    primary_group_id BIGINT,
    flair_group_id  BIGINT,
    locale          VARCHAR(10),
    last_seen_at    TIMESTAMPTZ,
    last_posted_at  TIMESTAMPTZ,
    last_emailed_at TIMESTAMPTZ,
    silenced_till   TIMESTAMPTZ,
    suspended_till  TIMESTAMPTZ,
    suspended_at    TIMESTAMPTZ,
    views           INTEGER NOT NULL DEFAULT 0,
    flag_level      SMALLINT NOT NULL DEFAULT 0,
    title           VARCHAR(255),
    uploaded_avatar_id BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_emails (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    email      VARCHAR(513) NOT NULL,
    "primary"  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, email)
);
CREATE UNIQUE INDEX idx_user_emails_primary ON user_emails(user_id) WHERE "primary" = TRUE;

CREATE TABLE user_profiles (
    user_id          BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    bio_raw          TEXT,
    bio_cooked       TEXT,
    bio_cooked_version SMALLINT,
    location         VARCHAR(3000),
    website          VARCHAR(3000),
    profile_background_upload_id BIGINT,
    card_background_upload_id    BIGINT,
    views            INTEGER NOT NULL DEFAULT 0,
    dismissed_banner_key VARCHAR(255)
);

CREATE TABLE user_stats (
    user_id                 BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    topics_entered          INTEGER NOT NULL DEFAULT 0,
    time_read               INTEGER NOT NULL DEFAULT 0,
    days_visited            INTEGER NOT NULL DEFAULT 0,
    posts_read_count        INTEGER NOT NULL DEFAULT 0,
    likes_given             INTEGER NOT NULL DEFAULT 0,
    likes_received          INTEGER NOT NULL DEFAULT 0,
    new_since               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    read_faq                TIMESTAMPTZ,
    first_post_created_at   TIMESTAMPTZ,
    post_count              INTEGER NOT NULL DEFAULT 0,
    topic_count             INTEGER NOT NULL DEFAULT 0,
    bounce_score            REAL NOT NULL DEFAULT 0,
    reset_bounce_score_after TIMESTAMPTZ,
    flags_agreed            INTEGER NOT NULL DEFAULT 0,
    flags_disagreed         INTEGER NOT NULL DEFAULT 0,
    flags_ignored           INTEGER NOT NULL DEFAULT 0,
    first_unread_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    distinct_badge_count    INTEGER NOT NULL DEFAULT 0,
    first_unread_pm_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    digest_attempted_at     TIMESTAMPTZ
);

CREATE TABLE user_options (
    user_id                           BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    email_level                       SMALLINT NOT NULL DEFAULT 1,
    email_messages_level              SMALLINT NOT NULL DEFAULT 0,
    email_digests                     BOOLEAN NOT NULL DEFAULT TRUE,
    digest_after_minutes              INTEGER NOT NULL DEFAULT 10080,
    mailing_list_mode                 BOOLEAN NOT NULL DEFAULT FALSE,
    mailing_list_mode_frequency       SMALLINT NOT NULL DEFAULT 1,
    external_links_in_new_tab         BOOLEAN NOT NULL DEFAULT FALSE,
    dark_scheme_id                    BIGINT,
    dynamic_favicon                   BOOLEAN NOT NULL DEFAULT FALSE,
    enable_quoting                    BOOLEAN NOT NULL DEFAULT TRUE,
    enable_defer                      BOOLEAN NOT NULL DEFAULT FALSE,
    automatically_unpin_topics        BOOLEAN NOT NULL DEFAULT TRUE,
    notification_level_when_replying  SMALLINT NOT NULL DEFAULT 2,
    new_topic_duration_minutes        INTEGER NOT NULL DEFAULT -1,
    auto_track_topics_after_msecs     INTEGER NOT NULL DEFAULT 240000,
    like_notification_frequency       SMALLINT NOT NULL DEFAULT 1,
    include_tl0_in_digests            BOOLEAN NOT NULL DEFAULT FALSE,
    theme_ids                         BIGINT[] NOT NULL DEFAULT '{}',
    text_size_key                     SMALLINT NOT NULL DEFAULT 0,
    title_count_mode_key              SMALLINT NOT NULL DEFAULT 0,
    timezone                          VARCHAR(50),
    skip_new_user_tips                BOOLEAN NOT NULL DEFAULT FALSE,
    default_calendar                  SMALLINT NOT NULL DEFAULT 0,
    oldest_search_log_date            TIMESTAMPTZ,
    seen_popups                       INTEGER[],
    sidebar_link_to_filtered_list     BOOLEAN NOT NULL DEFAULT FALSE,
    sidebar_show_count_of_new_items   BOOLEAN NOT NULL DEFAULT FALSE,
    chat_enabled                      BOOLEAN NOT NULL DEFAULT TRUE,
    chat_sound                        VARCHAR(100),
    chat_email_frequency              SMALLINT NOT NULL DEFAULT 1,
    chat_header_indicator_preference  SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE user_auth_tokens (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    auth_token      VARCHAR(64) NOT NULL UNIQUE,
    prev_auth_token VARCHAR(64) NOT NULL,
    user_agent      TEXT,
    auth_token_seen BOOLEAN NOT NULL DEFAULT FALSE,
    client_ip       INET,
    rotated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    seen_at         TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_second_factors (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    method     SMALLINT NOT NULL,
    data       TEXT NOT NULL,
    enabled    BOOLEAN NOT NULL DEFAULT TRUE,
    name       VARCHAR(255),
    last_used  TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_custom_fields (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name       VARCHAR(256) NOT NULL,
    value      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_ucf_user_name ON user_custom_fields(user_id, name);

CREATE TABLE user_fields (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,
    field_type      VARCHAR(50) NOT NULL,
    editable        BOOLEAN NOT NULL DEFAULT FALSE,
    description     VARCHAR(1000),
    required        BOOLEAN NOT NULL DEFAULT FALSE,
    show_on_profile BOOLEAN NOT NULL DEFAULT FALSE,
    show_on_user_card BOOLEAN NOT NULL DEFAULT FALSE,
    "position"      INTEGER NOT NULL DEFAULT 0,
    searchable      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_field_options (
    id            BIGSERIAL PRIMARY KEY,
    user_field_id BIGINT NOT NULL REFERENCES user_fields(id) ON DELETE CASCADE,
    value         VARCHAR(255) NOT NULL
);

CREATE TABLE user_actions (
    id                BIGSERIAL PRIMARY KEY,
    action_type       SMALLINT NOT NULL,
    user_id           BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_topic_id   BIGINT,
    target_post_id    BIGINT,
    target_user_id    BIGINT,
    acting_user_id    BIGINT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_user_actions_user ON user_actions(user_id, action_type, created_at DESC);

CREATE TABLE user_visits (
    id        BIGSERIAL PRIMARY KEY,
    user_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    visited_at DATE NOT NULL,
    posts_read INTEGER NOT NULL DEFAULT 0,
    mobile    BOOLEAN NOT NULL DEFAULT FALSE,
    time_read INTEGER NOT NULL DEFAULT 0,
    UNIQUE(user_id, visited_at)
);

CREATE TABLE user_avatars (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    custom_upload_id    BIGINT,
    gravatar_upload_id  BIGINT,
    last_gravatar_download_at TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_histories (
    id                BIGSERIAL PRIMARY KEY,
    action            SMALLINT NOT NULL,
    acting_user_id    BIGINT,
    target_user_id    BIGINT,
    details           TEXT,
    subject           TEXT,
    previous_value    TEXT,
    new_value         TEXT,
    topic_id          BIGINT,
    post_id           BIGINT,
    category_id       BIGINT,
    ip_address        INET,
    admin_only        BOOLEAN NOT NULL DEFAULT FALSE,
    custom_type       VARCHAR(255),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_api_keys (
    id                 BIGSERIAL PRIMARY KEY,
    user_id            BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id          VARCHAR(255) NOT NULL,
    application_name   VARCHAR(255) NOT NULL,
    push_url           VARCHAR(2000),
    key_hash           VARCHAR(64) NOT NULL,
    scopes             TEXT[],
    last_used_at       TIMESTAMPTZ,
    revoked_at         TIMESTAMPTZ,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_associated_accounts (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider_name   VARCHAR(255) NOT NULL,
    provider_uid    VARCHAR(255) NOT NULL,
    info            JSONB,
    last_used       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider_name, provider_uid)
);

CREATE TABLE single_sign_on_records (
    id                BIGSERIAL PRIMARY KEY,
    user_id           BIGINT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    external_id       VARCHAR(255) NOT NULL UNIQUE,
    external_username VARCHAR(255),
    external_email    VARCHAR(255),
    external_name     VARCHAR(255),
    external_avatar_url TEXT,
    last_payload      TEXT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_notification_schedules (
    id       BIGSERIAL PRIMARY KEY,
    user_id  BIGINT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    enabled  BOOLEAN NOT NULL DEFAULT FALSE,
    day_0_start_time SMALLINT NOT NULL DEFAULT 480,
    day_0_end_time   SMALLINT NOT NULL DEFAULT 1020,
    day_1_start_time SMALLINT NOT NULL DEFAULT 480,
    day_1_end_time   SMALLINT NOT NULL DEFAULT 1020,
    day_2_start_time SMALLINT NOT NULL DEFAULT 480,
    day_2_end_time   SMALLINT NOT NULL DEFAULT 1020,
    day_3_start_time SMALLINT NOT NULL DEFAULT 480,
    day_3_end_time   SMALLINT NOT NULL DEFAULT 1020,
    day_4_start_time SMALLINT NOT NULL DEFAULT 480,
    day_4_end_time   SMALLINT NOT NULL DEFAULT 1020,
    day_5_start_time SMALLINT NOT NULL DEFAULT 480,
    day_5_end_time   SMALLINT NOT NULL DEFAULT 1020,
    day_6_start_time SMALLINT NOT NULL DEFAULT 480,
    day_6_end_time   SMALLINT NOT NULL DEFAULT 1020
);

CREATE TABLE user_statuses (
    user_id      BIGINT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    emoji        VARCHAR(100) NOT NULL,
    description  VARCHAR(255) NOT NULL,
    ends_at      TIMESTAMPTZ,
    set_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE do_not_disturb_timings (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    starts_at   TIMESTAMPTZ NOT NULL,
    ends_at     TIMESTAMPTZ NOT NULL,
    scheduled   BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE push_subscriptions (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    data       TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE email_tokens (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    email       VARCHAR(513) NOT NULL,
    token_hash  VARCHAR(64) NOT NULL,
    confirmed   BOOLEAN NOT NULL DEFAULT FALSE,
    expired     BOOLEAN NOT NULL DEFAULT FALSE,
    scope       SMALLINT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE email_change_requests (
    id               BIGSERIAL PRIMARY KEY,
    user_id          BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    old_email        VARCHAR(513),
    new_email        VARCHAR(513) NOT NULL,
    old_email_token_id BIGINT,
    new_email_token_id BIGINT,
    change_state     SMALLINT NOT NULL DEFAULT 1,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE anonymous_users (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    master_user_id  BIGINT NOT NULL REFERENCES users(id),
    active          BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE muted_users (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    muted_user_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, muted_user_id)
);

CREATE TABLE ignored_users (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ignored_user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expiring_at     TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, ignored_user_id)
);

CREATE TABLE directory_items (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    period_type     SMALLINT NOT NULL,
    likes_received  INTEGER NOT NULL DEFAULT 0,
    likes_given     INTEGER NOT NULL DEFAULT 0,
    topics_entered  INTEGER NOT NULL DEFAULT 0,
    topic_count     INTEGER NOT NULL DEFAULT 0,
    post_count      INTEGER NOT NULL DEFAULT 0,
    days_visited    INTEGER NOT NULL DEFAULT 0,
    posts_read      INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 2. GROUPS SUBSYSTEM (before categories, because categories references groups)
-- =============================================================================

CREATE TABLE groups (
    id                           BIGSERIAL PRIMARY KEY,
    name                         VARCHAR(255) NOT NULL UNIQUE,
    automatic                    BOOLEAN NOT NULL DEFAULT FALSE,
    user_count                   INTEGER NOT NULL DEFAULT 0,
    automatic_membership_email_domains TEXT,
    primary_group                BOOLEAN NOT NULL DEFAULT FALSE,
    title                        VARCHAR(255),
    grant_trust_level            SMALLINT,
    bio_raw                      TEXT,
    bio_cooked                   TEXT,
    allow_membership_requests    BOOLEAN NOT NULL DEFAULT FALSE,
    full_name                    VARCHAR(255),
    default_notification_level   SMALLINT NOT NULL DEFAULT 3,
    visibility_level             SMALLINT NOT NULL DEFAULT 0,
    public_exit                  BOOLEAN NOT NULL DEFAULT FALSE,
    public_admission             BOOLEAN NOT NULL DEFAULT FALSE,
    membership_request_template  TEXT,
    messageable_level            SMALLINT NOT NULL DEFAULT 0,
    mentionable_level            SMALLINT NOT NULL DEFAULT 0,
    members_visibility_level     SMALLINT NOT NULL DEFAULT 0,
    flair_upload_id              BIGINT,
    flair_bg_color               VARCHAR(6),
    flair_color                  VARCHAR(6),
    publish_read_state           BOOLEAN NOT NULL DEFAULT FALSE,
    smtp_server                  VARCHAR(255),
    smtp_port                    INTEGER,
    smtp_ssl                     BOOLEAN,
    smtp_enabled                 BOOLEAN NOT NULL DEFAULT FALSE,
    smtp_updated_at              TIMESTAMPTZ,
    smtp_updated_by_id           BIGINT,
    imap_server                  VARCHAR(255),
    imap_port                    INTEGER,
    imap_ssl                     BOOLEAN,
    imap_enabled                 BOOLEAN NOT NULL DEFAULT FALSE,
    imap_mailbox_name            VARCHAR(255) NOT NULL DEFAULT 'INBOX',
    imap_uid_validity            INTEGER NOT NULL DEFAULT 0,
    imap_last_uid                INTEGER NOT NULL DEFAULT 0,
    email_username               VARCHAR(255),
    email_password               VARCHAR(500),
    imap_last_error              TEXT,
    imap_old_emails              INTEGER,
    imap_new_emails              INTEGER,
    created_at                   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE group_users (
    id                    BIGSERIAL PRIMARY KEY,
    group_id              BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id               BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    owner                 BOOLEAN NOT NULL DEFAULT FALSE,
    notification_level    SMALLINT NOT NULL DEFAULT 2,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(group_id, user_id)
);

CREATE TABLE group_custom_fields (
    id         BIGSERIAL PRIMARY KEY,
    group_id   BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    name       VARCHAR(256) NOT NULL,
    value      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE group_histories (
    id              BIGSERIAL PRIMARY KEY,
    group_id        BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    acting_user_id  BIGINT NOT NULL,
    target_user_id  BIGINT,
    action          SMALLINT NOT NULL,
    subject         VARCHAR(255),
    prev_value      TEXT,
    new_value       TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE group_requests (
    id         BIGSERIAL PRIMARY KEY,
    group_id   BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    reason     TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 3. UPLOADS SUBSYSTEM (before categories/topics/posts that reference uploads)
-- =============================================================================

CREATE TABLE uploads (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT REFERENCES users(id) ON DELETE SET NULL,
    original_filename   VARCHAR(512) NOT NULL,
    filesize            BIGINT NOT NULL,
    width               INTEGER,
    height              INTEGER,
    url                 VARCHAR(2000) NOT NULL,
    sha1                VARCHAR(40),
    origin              VARCHAR(1000),
    retain_hours        INTEGER,
    extension           VARCHAR(10),
    thumbnail_width     INTEGER,
    thumbnail_height    INTEGER,
    etag                VARCHAR(100),
    secure              BOOLEAN NOT NULL DEFAULT FALSE,
    access_control_post_id BIGINT,
    original_sha1       VARCHAR(40),
    animated            BOOLEAN,
    verification_status SMALLINT NOT NULL DEFAULT 1,
    security_last_changed_at TIMESTAMPTZ,
    security_last_changed_reason VARCHAR(100),
    dominant_color      VARCHAR(6),
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_uploads_sha1 ON uploads(sha1);
CREATE INDEX idx_uploads_url ON uploads(url);

CREATE TABLE upload_references (
    id          BIGSERIAL PRIMARY KEY,
    upload_id   BIGINT NOT NULL REFERENCES uploads(id) ON DELETE CASCADE,
    target_type VARCHAR(60) NOT NULL,
    target_id   BIGINT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_upload_references_target ON upload_references(target_type, target_id);

CREATE TABLE optimized_images (
    id          BIGSERIAL PRIMARY KEY,
    upload_id   BIGINT NOT NULL REFERENCES uploads(id) ON DELETE CASCADE,
    sha1        VARCHAR(40) NOT NULL,
    extension   VARCHAR(10) NOT NULL,
    width       INTEGER NOT NULL,
    height      INTEGER NOT NULL,
    url         VARCHAR(2000) NOT NULL,
    filesize    INTEGER,
    etag        VARCHAR(100),
    version     INTEGER,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_optimized_images_upload ON optimized_images(upload_id);

-- =============================================================================
-- 4. BADGES SUBSYSTEM
-- =============================================================================

CREATE TABLE badge_types (
    id         BIGSERIAL PRIMARY KEY,
    name       VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE badge_groupings (
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT,
    "position"  INTEGER NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE badges (
    id                  BIGSERIAL PRIMARY KEY,
    name                VARCHAR(255) NOT NULL UNIQUE,
    description         TEXT,
    badge_type_id       BIGINT NOT NULL REFERENCES badge_types(id),
    badge_grouping_id   BIGINT NOT NULL REFERENCES badge_groupings(id),
    grant_count         INTEGER NOT NULL DEFAULT 0,
    allow_title         BOOLEAN NOT NULL DEFAULT FALSE,
    multiple_grant      BOOLEAN NOT NULL DEFAULT FALSE,
    icon                VARCHAR(255),
    listable            BOOLEAN NOT NULL DEFAULT TRUE,
    target_posts        BOOLEAN NOT NULL DEFAULT FALSE,
    query               TEXT,
    enabled             BOOLEAN NOT NULL DEFAULT TRUE,
    auto_revoke         BOOLEAN NOT NULL DEFAULT TRUE,
    badge_grouping_position INTEGER NOT NULL DEFAULT 0,
    trigger_type        SMALLINT,
    show_posts          BOOLEAN NOT NULL DEFAULT FALSE,
    system              BOOLEAN NOT NULL DEFAULT FALSE,
    long_description    TEXT,
    image_upload_id     BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_badges (
    id                BIGSERIAL PRIMARY KEY,
    badge_id          BIGINT NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    user_id           BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    granted_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by_id     BIGINT NOT NULL REFERENCES users(id),
    post_id           BIGINT,
    notification_id   BIGINT,
    seq               INTEGER NOT NULL DEFAULT 0,
    featured_rank     INTEGER,
    is_favorite       BOOLEAN,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_user_badges_user ON user_badges(user_id);
CREATE INDEX idx_user_badges_badge ON user_badges(badge_id);

-- =============================================================================
-- 5. CATEGORIES SUBSYSTEM
-- =============================================================================

CREATE TABLE categories (
    id                          BIGSERIAL PRIMARY KEY,
    name                        VARCHAR(255) NOT NULL,
    color                       VARCHAR(6) NOT NULL DEFAULT '0088CC',
    text_color                  VARCHAR(6) NOT NULL DEFAULT 'FFFFFF',
    topic_id                    BIGINT,
    topic_count                 INTEGER NOT NULL DEFAULT 0,
    description                 TEXT,
    description_text            TEXT,
    description_excerpt         TEXT,
    slug                        VARCHAR(255) NOT NULL,
    user_id                     BIGINT NOT NULL REFERENCES users(id),
    parent_category_id          BIGINT REFERENCES categories(id),
    "position"                  INTEGER,
    logo_upload_id              BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    background_upload_id        BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    post_count                  INTEGER NOT NULL DEFAULT 0,
    latest_post_id              BIGINT,
    latest_topic_id             BIGINT,
    topics_day                  INTEGER NOT NULL DEFAULT 0,
    topics_week                 INTEGER NOT NULL DEFAULT 0,
    topics_month                INTEGER NOT NULL DEFAULT 0,
    topics_year                 INTEGER NOT NULL DEFAULT 0,
    topics_all_time             INTEGER NOT NULL DEFAULT 0,
    posts_day                   INTEGER NOT NULL DEFAULT 0,
    posts_week                  INTEGER NOT NULL DEFAULT 0,
    posts_month                 INTEGER NOT NULL DEFAULT 0,
    posts_year                  INTEGER NOT NULL DEFAULT 0,
    posts_all_time              INTEGER NOT NULL DEFAULT 0,
    num_featured_topics         INTEGER NOT NULL DEFAULT 3,
    suppress_from_latest        BOOLEAN NOT NULL DEFAULT FALSE,
    all_topics_wiki             BOOLEAN NOT NULL DEFAULT FALSE,
    allow_badges                BOOLEAN NOT NULL DEFAULT TRUE,
    name_lower                  VARCHAR(255) NOT NULL,
    auto_close_based_on_last_post BOOLEAN NOT NULL DEFAULT FALSE,
    auto_close_hours            REAL,
    topic_template              TEXT,
    contains_messages           BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order                  VARCHAR(50),
    sort_ascending              BOOLEAN,
    allow_global_tags           BOOLEAN NOT NULL DEFAULT FALSE,
    default_view                VARCHAR(50),
    default_top_period          VARCHAR(20) NOT NULL DEFAULT 'all',
    default_slow_mode_seconds   INTEGER,
    minimum_required_tags       SMALLINT NOT NULL DEFAULT 0,
    navigate_to_first_post_after_read BOOLEAN NOT NULL DEFAULT FALSE,
    search_priority             SMALLINT NOT NULL DEFAULT 0,
    allow_unlimited_owner_edits_on_first_post BOOLEAN NOT NULL DEFAULT FALSE,
    default_list_filter         VARCHAR(20) NOT NULL DEFAULT 'all',
    read_restricted             BOOLEAN NOT NULL DEFAULT FALSE,
    reviewable_by_group_id      BIGINT REFERENCES groups(id) ON DELETE SET NULL,
    read_only_banner            TEXT,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX idx_categories_name_parent ON categories(COALESCE(parent_category_id, -1), name);
CREATE INDEX idx_categories_slug ON categories(slug);

CREATE TABLE category_custom_fields (
    id          BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    name        VARCHAR(256) NOT NULL,
    value       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_ccf_category_name ON category_custom_fields(category_id, name);

CREATE TABLE category_groups (
    id              BIGSERIAL PRIMARY KEY,
    category_id     BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    group_id        BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    permission_type SMALLINT NOT NULL DEFAULT 1,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX idx_category_groups_unique ON category_groups(category_id, group_id, permission_type);

CREATE TABLE category_users (
    id                  BIGSERIAL PRIMARY KEY,
    category_id         BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    user_id             BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_level  SMALLINT NOT NULL,
    last_seen_at        TIMESTAMPTZ,
    UNIQUE(user_id, category_id)
);

CREATE TABLE category_featured_topics (
    id          BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    topic_id    BIGINT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "rank"      INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE category_search_data (
    category_id BIGINT PRIMARY KEY REFERENCES categories(id) ON DELETE CASCADE,
    search_data TSVECTOR,
    raw_data    TEXT,
    locale      VARCHAR(10)
);
CREATE INDEX idx_category_search_data ON category_search_data USING GIN(search_data);

CREATE TABLE category_tag_stats (
    id          BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL,
    tag_id      BIGINT NOT NULL,
    topic_count INTEGER NOT NULL DEFAULT 0,
    UNIQUE(category_id, tag_id)
);

-- =============================================================================
-- 6. TAGS SUBSYSTEM
-- =============================================================================

CREATE TABLE tags (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL UNIQUE,
    topic_count     INTEGER NOT NULL DEFAULT 0,
    pm_topic_count  INTEGER NOT NULL DEFAULT 0,
    target_tag_id   BIGINT REFERENCES tags(id) ON DELETE SET NULL,
    description     TEXT,
    public_topic_count INTEGER NOT NULL DEFAULT 0,
    staff_topic_count  INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_tags_name_lower ON tags(LOWER(name));

CREATE TABLE tag_groups (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,
    parent_tag_id   BIGINT REFERENCES tags(id) ON DELETE SET NULL,
    one_per_topic   BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tag_group_memberships (
    id              BIGSERIAL PRIMARY KEY,
    tag_id          BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    tag_group_id    BIGINT NOT NULL REFERENCES tag_groups(id) ON DELETE CASCADE,
    UNIQUE(tag_id, tag_group_id)
);

CREATE TABLE tag_group_permissions (
    id              BIGSERIAL PRIMARY KEY,
    tag_group_id    BIGINT NOT NULL REFERENCES tag_groups(id) ON DELETE CASCADE,
    group_id        BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    permission_type SMALLINT NOT NULL DEFAULT 1,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE category_tags (
    id          BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    tag_id      BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(category_id, tag_id)
);

CREATE TABLE category_tag_groups (
    id            BIGSERIAL PRIMARY KEY,
    category_id   BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    tag_group_id  BIGINT NOT NULL REFERENCES tag_groups(id) ON DELETE CASCADE,
    UNIQUE(category_id, tag_group_id)
);

CREATE TABLE category_required_tag_groups (
    id            BIGSERIAL PRIMARY KEY,
    category_id   BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    tag_group_id  BIGINT NOT NULL REFERENCES tag_groups(id) ON DELETE CASCADE,
    min_count     INTEGER NOT NULL DEFAULT 1,
    "order"       INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE tag_users (
    id                 BIGSERIAL PRIMARY KEY,
    tag_id             BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    user_id            BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_level SMALLINT NOT NULL,
    UNIQUE(tag_id, user_id)
);

-- =============================================================================
-- 7. TOPICS SUBSYSTEM
-- =============================================================================

CREATE TABLE topics (
    id                          BIGSERIAL PRIMARY KEY,
    title                       VARCHAR(255) NOT NULL,
    last_posted_at              TIMESTAMPTZ,
    views                       INTEGER NOT NULL DEFAULT 0,
    posts_count                 INTEGER NOT NULL DEFAULT 0,
    reply_count                 INTEGER NOT NULL DEFAULT 0,
    user_id                     BIGINT NOT NULL REFERENCES users(id),
    highest_post_number         INTEGER NOT NULL DEFAULT 0,
    highest_staff_post_number   INTEGER NOT NULL DEFAULT 0,
    like_count                  INTEGER NOT NULL DEFAULT 0,
    incoming_link_count         INTEGER NOT NULL DEFAULT 0,
    category_id                 BIGINT REFERENCES categories(id),
    visible                     BOOLEAN NOT NULL DEFAULT TRUE,
    moderator_posts_count       INTEGER NOT NULL DEFAULT 0,
    closed                      BOOLEAN NOT NULL DEFAULT FALSE,
    archived                    BOOLEAN NOT NULL DEFAULT FALSE,
    bumped_at                   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    has_summary                 BOOLEAN NOT NULL DEFAULT FALSE,
    archetype                   VARCHAR(255) NOT NULL DEFAULT 'regular',
    featured_user1_id           BIGINT REFERENCES users(id),
    featured_user2_id           BIGINT REFERENCES users(id),
    featured_user3_id           BIGINT REFERENCES users(id),
    featured_user4_id           BIGINT REFERENCES users(id),
    notify_moderators_count     INTEGER NOT NULL DEFAULT 0,
    spam_count                  INTEGER NOT NULL DEFAULT 0,
    pinned_at                   TIMESTAMPTZ,
    score                       REAL,
    percent_rank                REAL NOT NULL DEFAULT 1.0,
    subtype                     VARCHAR(255),
    slug                        VARCHAR(255),
    deleted_at                  TIMESTAMPTZ,
    deleted_by_id               BIGINT REFERENCES users(id),
    participant_count           INTEGER NOT NULL DEFAULT 1,
    word_count                  INTEGER,
    excerpt                     VARCHAR(1000),
    pinned_globally             BOOLEAN NOT NULL DEFAULT FALSE,
    pinned_until                TIMESTAMPTZ,
    fancy_title                 VARCHAR(400),
    image_upload_id             BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    slow_mode_seconds           INTEGER NOT NULL DEFAULT 0,
    featured_link               VARCHAR(2000),
    external_id                 VARCHAR(255),
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_topics_category ON topics(category_id) WHERE visible AND deleted_at IS NULL;
CREATE INDEX idx_topics_created_at ON topics(created_at DESC);
CREATE INDEX idx_topics_bumped_at ON topics(bumped_at DESC) WHERE deleted_at IS NULL;
CREATE INDEX idx_topics_slug ON topics(slug);
CREATE INDEX idx_topics_user ON topics(user_id);
CREATE INDEX idx_topics_external ON topics(external_id) WHERE external_id IS NOT NULL;

CREATE TABLE topic_custom_fields (
    id         BIGSERIAL PRIMARY KEY,
    topic_id   BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    name       VARCHAR(256) NOT NULL,
    value      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_tcf_topic_name ON topic_custom_fields(topic_id, name);

CREATE TABLE topic_users (
    id                      BIGSERIAL PRIMARY KEY,
    user_id                 BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id                BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    posted                  BOOLEAN NOT NULL DEFAULT FALSE,
    last_read_post_number   INTEGER,
    last_visited_at         TIMESTAMPTZ,
    first_visited_at        TIMESTAMPTZ,
    notification_level      SMALLINT NOT NULL DEFAULT 1,
    notifications_changed_at TIMESTAMPTZ,
    notifications_reason_id SMALLINT,
    total_msecs_viewed      INTEGER NOT NULL DEFAULT 0,
    cleared_pinned_at       TIMESTAMPTZ,
    bookmarked              BOOLEAN NOT NULL DEFAULT FALSE,
    liked                   BOOLEAN NOT NULL DEFAULT FALSE,
    last_emailed_post_number INTEGER,
    UNIQUE(user_id, topic_id)
);

CREATE TABLE topic_tags (
    id        BIGSERIAL PRIMARY KEY,
    topic_id  BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    tag_id    BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    UNIQUE(topic_id, tag_id)
);
CREATE INDEX idx_topic_tags_tag ON topic_tags(tag_id);

CREATE TABLE topic_views (
    id          BIGSERIAL PRIMARY KEY,
    topic_id    BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    viewed_at   DATE NOT NULL,
    user_id     BIGINT REFERENCES users(id) ON DELETE CASCADE,
    ip_address  INET,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_topic_views_topic_date ON topic_views(topic_id, viewed_at);
CREATE INDEX idx_topic_views_user ON topic_views(user_id) WHERE user_id IS NOT NULL;
CREATE UNIQUE INDEX idx_topic_views_unique_user ON topic_views(topic_id, user_id, viewed_at) WHERE user_id IS NOT NULL;
CREATE UNIQUE INDEX idx_topic_views_unique_ip ON topic_views(topic_id, ip_address, viewed_at) WHERE user_id IS NULL;

CREATE TABLE topic_links (
    id              BIGSERIAL PRIMARY KEY,
    topic_id        BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    post_id         BIGINT,
    user_id         BIGINT NOT NULL REFERENCES users(id),
    url             VARCHAR(500) NOT NULL,
    domain          VARCHAR(255) NOT NULL,
    internal        BOOLEAN NOT NULL DEFAULT FALSE,
    link_topic_id   BIGINT REFERENCES topics(id) ON DELETE SET NULL,
    link_post_id    BIGINT,
    clicks          INTEGER NOT NULL DEFAULT 0,
    reflection      BOOLEAN NOT NULL DEFAULT FALSE,
    title           VARCHAR(255),
    crawled_at      TIMESTAMPTZ,
    quote           BOOLEAN NOT NULL DEFAULT FALSE,
    extension       VARCHAR(10),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_topic_links_topic ON topic_links(topic_id);

CREATE TABLE topic_link_clicks (
    id              BIGSERIAL PRIMARY KEY,
    topic_link_id   BIGINT NOT NULL REFERENCES topic_links(id) ON DELETE CASCADE,
    user_id         BIGINT REFERENCES users(id) ON DELETE SET NULL,
    ip_address      INET,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE topic_timers (
    id                BIGSERIAL PRIMARY KEY,
    execute_at        TIMESTAMPTZ NOT NULL,
    status_type       SMALLINT NOT NULL,
    user_id           BIGINT NOT NULL REFERENCES users(id),
    topic_id          BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    based_on_last_post BOOLEAN NOT NULL DEFAULT FALSE,
    deleted_at        TIMESTAMPTZ,
    deleted_by_id     BIGINT REFERENCES users(id),
    category_id       BIGINT REFERENCES categories(id),
    public_type       BOOLEAN NOT NULL DEFAULT TRUE,
    duration_minutes  INTEGER,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE topic_allowed_users (
    id        BIGSERIAL PRIMARY KEY,
    user_id   BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id  BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, topic_id)
);

CREATE TABLE topic_allowed_groups (
    id        BIGSERIAL PRIMARY KEY,
    group_id  BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    topic_id  BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    UNIQUE(group_id, topic_id)
);

CREATE TABLE topic_search_data (
    topic_id    BIGINT PRIMARY KEY REFERENCES topics(id) ON DELETE CASCADE,
    search_data TSVECTOR,
    raw_data    TEXT,
    locale      VARCHAR(10)
);
CREATE INDEX idx_topic_search_data ON topic_search_data USING GIN(search_data);

CREATE TABLE topic_thumbnails (
    id                  BIGSERIAL PRIMARY KEY,
    upload_id           BIGINT NOT NULL REFERENCES uploads(id) ON DELETE CASCADE,
    optimized_image_id  BIGINT REFERENCES optimized_images(id) ON DELETE SET NULL,
    max_width           INTEGER NOT NULL,
    max_height          INTEGER NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE topic_embeds (
    id          BIGSERIAL PRIMARY KEY,
    topic_id    BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    embed_url   VARCHAR(1000) NOT NULL UNIQUE,
    post_id     BIGINT,
    content_sha1 VARCHAR(40),
    deleted_at  TIMESTAMPTZ,
    deleted_by_id BIGINT REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 8. POSTS SUBSYSTEM
-- =============================================================================

CREATE TABLE posts (
    id                      BIGSERIAL PRIMARY KEY,
    user_id                 BIGINT NOT NULL REFERENCES users(id),
    topic_id                BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    post_number             INTEGER NOT NULL,
    raw                     TEXT NOT NULL,
    cooked                  TEXT NOT NULL,
    reply_to_post_number    INTEGER,
    reply_count             INTEGER NOT NULL DEFAULT 0,
    quote_count             INTEGER NOT NULL DEFAULT 0,
    deleted_at              TIMESTAMPTZ,
    off_topic_count         INTEGER NOT NULL DEFAULT 0,
    like_count              INTEGER NOT NULL DEFAULT 0,
    incoming_link_count     INTEGER NOT NULL DEFAULT 0,
    bookmark_count          INTEGER NOT NULL DEFAULT 0,
    score                   REAL,
    reads                   INTEGER NOT NULL DEFAULT 0,
    post_type               SMALLINT NOT NULL DEFAULT 1,
    sort_order              INTEGER,
    last_editor_id          BIGINT REFERENCES users(id),
    hidden                  BOOLEAN NOT NULL DEFAULT FALSE,
    hidden_reason_id        SMALLINT,
    hidden_at               TIMESTAMPTZ,
    notify_moderators_count INTEGER NOT NULL DEFAULT 0,
    spam_count              INTEGER NOT NULL DEFAULT 0,
    illegal_count           INTEGER NOT NULL DEFAULT 0,
    inappropriate_count     INTEGER NOT NULL DEFAULT 0,
    last_version_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_deleted            BOOLEAN NOT NULL DEFAULT FALSE,
    reply_to_user_id        BIGINT REFERENCES users(id),
    percent_rank            REAL,
    notify_user_count       INTEGER NOT NULL DEFAULT 0,
    like_score              SMALLINT NOT NULL DEFAULT 0,
    deleted_by_id           BIGINT REFERENCES users(id),
    edit_reason             VARCHAR(1000),
    word_count              INTEGER,
    version                 INTEGER NOT NULL DEFAULT 1,
    cook_method             SMALLINT NOT NULL DEFAULT 1,
    wiki                    BOOLEAN NOT NULL DEFAULT FALSE,
    baked_at                TIMESTAMPTZ,
    baked_version           INTEGER,
    via_email               BOOLEAN NOT NULL DEFAULT FALSE,
    raw_email               TEXT,
    public_version          INTEGER NOT NULL DEFAULT 1,
    action_code             VARCHAR(100),
    locked_by_id            BIGINT REFERENCES users(id),
    image_upload_id         BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    outbound_message_id     VARCHAR(255),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_posts_topic ON posts(topic_id, post_number);
CREATE INDEX idx_posts_user ON posts(user_id);
CREATE INDEX idx_posts_created_at ON posts(created_at);
CREATE UNIQUE INDEX idx_posts_topic_post_number ON posts(topic_id, post_number);
CREATE INDEX idx_posts_reply_to ON posts(topic_id, reply_to_post_number) WHERE reply_to_post_number IS NOT NULL;

CREATE TABLE post_custom_fields (
    id        BIGSERIAL PRIMARY KEY,
    post_id   BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    name      VARCHAR(256) NOT NULL,
    value     TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_pcf_post_name ON post_custom_fields(post_id, name);

CREATE TABLE post_actions (
    id                BIGSERIAL PRIMARY KEY,
    post_id           BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id           BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_action_type_id SMALLINT NOT NULL,
    deleted_at        TIMESTAMPTZ,
    deleted_by_id     BIGINT REFERENCES users(id),
    related_post_id   BIGINT REFERENCES posts(id) ON DELETE SET NULL,
    staff_took_action BOOLEAN NOT NULL DEFAULT FALSE,
    deferred_by_id    BIGINT REFERENCES users(id),
    targets_topic     BOOLEAN NOT NULL DEFAULT FALSE,
    agreed_at         TIMESTAMPTZ,
    agreed_by_id      BIGINT REFERENCES users(id),
    deferred_at       TIMESTAMPTZ,
    disagreed_at      TIMESTAMPTZ,
    disagreed_by_id   BIGINT REFERENCES users(id),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX idx_post_actions_unique ON post_actions(user_id, post_action_type_id, post_id, targets_topic) WHERE deleted_at IS NULL;

CREATE TABLE post_action_types (
    id            BIGSERIAL PRIMARY KEY,
    name_key      VARCHAR(50) NOT NULL,
    is_flag       BOOLEAN NOT NULL DEFAULT FALSE,
    icon          VARCHAR(50),
    "position"    INTEGER NOT NULL DEFAULT 0,
    score_bonus   REAL NOT NULL DEFAULT 0,
    reviewable_priority SMALLINT NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE post_replies (
    post_id       BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    reply_post_id BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (post_id, reply_post_id)
);

CREATE TABLE post_revisions (
    id            BIGSERIAL PRIMARY KEY,
    user_id       BIGINT REFERENCES users(id),
    post_id       BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    modifications JSONB,
    number        INTEGER NOT NULL,
    hidden        BOOLEAN NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_post_revisions_post ON post_revisions(post_id, number);

CREATE TABLE post_search_data (
    post_id     BIGINT PRIMARY KEY REFERENCES posts(id) ON DELETE CASCADE,
    search_data TSVECTOR,
    raw_data    TEXT,
    locale      VARCHAR(10),
    version     INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX idx_post_search_data ON post_search_data USING GIN(search_data);

CREATE TABLE post_uploads (
    id        BIGSERIAL PRIMARY KEY,
    post_id   BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    upload_id BIGINT NOT NULL REFERENCES uploads(id) ON DELETE CASCADE,
    UNIQUE(post_id, upload_id)
);

CREATE TABLE post_details (
    id         BIGSERIAL PRIMARY KEY,
    post_id    BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    key        VARCHAR(255) NOT NULL,
    value      TEXT,
    extra      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, key)
);

CREATE TABLE post_timings (
    topic_id    BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    post_number INTEGER NOT NULL,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    msecs       INTEGER NOT NULL,
    PRIMARY KEY (topic_id, post_number, user_id)
);

CREATE TABLE incoming_links (
    id              BIGSERIAL PRIMARY KEY,
    topic_id        BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    post_id         BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id         BIGINT REFERENCES users(id) ON DELETE SET NULL,
    ip_address      INET,
    current_user_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
    referer         VARCHAR(1000),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_incoming_links_post ON incoming_links(post_id);

CREATE TABLE quoted_posts (
    id             BIGSERIAL PRIMARY KEY,
    post_id        BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    quoted_post_id BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, quoted_post_id)
);

-- =============================================================================
-- 9. NOTIFICATIONS SUBSYSTEM
-- =============================================================================

CREATE TABLE notifications (
    id                BIGSERIAL PRIMARY KEY,
    notification_type SMALLINT NOT NULL,
    user_id           BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    data              JSONB NOT NULL,
    read              BOOLEAN NOT NULL DEFAULT FALSE,
    topic_id          BIGINT REFERENCES topics(id) ON DELETE CASCADE,
    post_number       INTEGER,
    post_action_id    BIGINT,
    high_priority     BOOLEAN NOT NULL DEFAULT FALSE,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_notifications_user ON notifications(user_id, created_at DESC);
CREATE INDEX idx_notifications_user_unread ON notifications(user_id, id DESC) WHERE NOT read;

-- =============================================================================
-- 10. BOOKMARKS SUBSYSTEM
-- =============================================================================

CREATE TABLE bookmarks (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    bookmarkable_type   VARCHAR(100) NOT NULL,
    bookmarkable_id     BIGINT NOT NULL,
    name                VARCHAR(100),
    reminder_at         TIMESTAMPTZ,
    reminder_last_sent_at TIMESTAMPTZ,
    reminder_set_at     TIMESTAMPTZ,
    auto_delete_preference SMALLINT NOT NULL DEFAULT 0,
    pinned              BOOLEAN NOT NULL DEFAULT FALSE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_bookmarks_user ON bookmarks(user_id);
CREATE UNIQUE INDEX idx_bookmarks_unique ON bookmarks(user_id, bookmarkable_type, bookmarkable_id);

-- =============================================================================
-- 11. DRAFTS SUBSYSTEM
-- =============================================================================

CREATE TABLE drafts (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    draft_key   VARCHAR(255) NOT NULL,
    data        TEXT NOT NULL,
    sequence    BIGINT NOT NULL DEFAULT 0,
    revisions   INTEGER NOT NULL DEFAULT 1,
    owner       VARCHAR(255),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, draft_key)
);

-- =============================================================================
-- 12. INVITES SUBSYSTEM
-- =============================================================================

CREATE TABLE invites (
    id                    BIGSERIAL PRIMARY KEY,
    invite_key            VARCHAR(64) NOT NULL UNIQUE,
    email                 VARCHAR(513),
    invited_by_id         BIGINT NOT NULL REFERENCES users(id),
    redemption_count      INTEGER NOT NULL DEFAULT 0,
    max_redemptions_allowed INTEGER NOT NULL DEFAULT 1,
    redeemed_at           TIMESTAMPTZ,
    expires_at            TIMESTAMPTZ NOT NULL,
    custom_message        TEXT,
    emailed_status        SMALLINT,
    domain                VARCHAR(255),
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE invited_users (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    invite_id  BIGINT NOT NULL REFERENCES invites(id) ON DELETE CASCADE,
    redeemed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE invited_groups (
    id         BIGSERIAL PRIMARY KEY,
    group_id   BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    invite_id  BIGINT NOT NULL REFERENCES invites(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE invite_links (
    id              BIGSERIAL PRIMARY KEY,
    invite_id       BIGINT NOT NULL REFERENCES invites(id) ON DELETE CASCADE,
    topic_id        BIGINT REFERENCES topics(id) ON DELETE SET NULL,
    group_id        BIGINT REFERENCES groups(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 13. EMAIL SUBSYSTEM
-- =============================================================================

CREATE TABLE email_logs (
    id              BIGSERIAL PRIMARY KEY,
    to_address      VARCHAR(513) NOT NULL,
    email_type      VARCHAR(200) NOT NULL,
    user_id         BIGINT REFERENCES users(id) ON DELETE SET NULL,
    post_id         BIGINT REFERENCES posts(id) ON DELETE SET NULL,
    bounce_key      UUID,
    bounced         BOOLEAN NOT NULL DEFAULT FALSE,
    message_id      VARCHAR(255),
    smtp_group_id   BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_email_logs_user ON email_logs(user_id);
CREATE INDEX idx_email_logs_bounce_key ON email_logs(bounce_key) WHERE bounce_key IS NOT NULL;

CREATE TABLE skipped_email_logs (
    id              BIGSERIAL PRIMARY KEY,
    email_type      VARCHAR(200) NOT NULL,
    to_address      VARCHAR(513) NOT NULL,
    user_id         BIGINT REFERENCES users(id) ON DELETE SET NULL,
    post_id         BIGINT REFERENCES posts(id) ON DELETE SET NULL,
    reason_type     SMALLINT NOT NULL,
    custom_reason   TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE incoming_emails (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT REFERENCES users(id) ON DELETE SET NULL,
    topic_id            BIGINT REFERENCES topics(id) ON DELETE SET NULL,
    post_id             BIGINT REFERENCES posts(id) ON DELETE SET NULL,
    raw                 TEXT,
    error               TEXT,
    message_id          VARCHAR(1000),
    from_address        VARCHAR(513),
    to_addresses        TEXT,
    cc_addresses        TEXT,
    subject             TEXT,
    rejection_message   TEXT,
    is_auto_generated   BOOLEAN NOT NULL DEFAULT FALSE,
    is_bounce           BOOLEAN NOT NULL DEFAULT FALSE,
    imap_uid_validity   INTEGER,
    imap_uid            INTEGER,
    imap_sync           BOOLEAN,
    imap_group_id       BIGINT,
    imap_missing         BOOLEAN NOT NULL DEFAULT FALSE,
    created_via         SMALLINT NOT NULL DEFAULT 0,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_incoming_emails_message_id ON incoming_emails(message_id);

-- =============================================================================
-- 14. MODERATION / REVIEWABLES SUBSYSTEM
-- =============================================================================

CREATE TABLE reviewables (
    id                    BIGSERIAL PRIMARY KEY,
    type                  VARCHAR(255) NOT NULL,
    status                SMALLINT NOT NULL DEFAULT 0,
    created_by_id         BIGINT NOT NULL REFERENCES users(id),
    reviewable_by_moderator BOOLEAN NOT NULL DEFAULT FALSE,
    reviewable_by_group_id BIGINT REFERENCES groups(id) ON DELETE SET NULL,
    category_id           BIGINT REFERENCES categories(id) ON DELETE SET NULL,
    topic_id              BIGINT REFERENCES topics(id) ON DELETE SET NULL,
    score                 REAL NOT NULL DEFAULT 0,
    potential_spam        BOOLEAN NOT NULL DEFAULT FALSE,
    target_type           VARCHAR(100),
    target_id             BIGINT,
    target_created_by_id  BIGINT REFERENCES users(id),
    payload               JSONB,
    version               INTEGER NOT NULL DEFAULT 0,
    latest_score          TIMESTAMPTZ,
    force_review          BOOLEAN NOT NULL DEFAULT FALSE,
    reject_reason         TEXT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_reviewables_status ON reviewables(status);
CREATE INDEX idx_reviewables_target ON reviewables(target_type, target_id);

CREATE TABLE reviewable_scores (
    id                      BIGSERIAL PRIMARY KEY,
    reviewable_id           BIGINT NOT NULL REFERENCES reviewables(id) ON DELETE CASCADE,
    user_id                 BIGINT NOT NULL REFERENCES users(id),
    reviewable_score_type   SMALLINT NOT NULL,
    status                  SMALLINT NOT NULL DEFAULT 0,
    score                   REAL NOT NULL DEFAULT 0,
    take_action_bonus       REAL NOT NULL DEFAULT 0,
    reviewed_by_id          BIGINT REFERENCES users(id),
    reviewed_at             TIMESTAMPTZ,
    meta_topic_id           BIGINT,
    reason                  TEXT,
    user_accuracy_bonus     REAL NOT NULL DEFAULT 0,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE reviewable_histories (
    id              BIGSERIAL PRIMARY KEY,
    reviewable_id   BIGINT NOT NULL REFERENCES reviewables(id) ON DELETE CASCADE,
    reviewable_history_type SMALLINT NOT NULL,
    status          SMALLINT NOT NULL DEFAULT 0,
    created_by_id   BIGINT NOT NULL REFERENCES users(id),
    edited           JSONB,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE reviewable_claimed_topics (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id),
    topic_id    BIGINT NOT NULL UNIQUE REFERENCES topics(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Screened tables
CREATE TABLE screened_emails (
    id              BIGSERIAL PRIMARY KEY,
    email           VARCHAR(513) NOT NULL,
    action_type     SMALLINT NOT NULL DEFAULT 0,
    match_count     INTEGER NOT NULL DEFAULT 0,
    last_match_at   TIMESTAMPTZ,
    ip_address      INET,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_screened_emails_email ON screened_emails(email);

CREATE TABLE screened_ip_addresses (
    id              BIGSERIAL PRIMARY KEY,
    ip_address      INET NOT NULL,
    action_type     SMALLINT NOT NULL DEFAULT 0,
    match_count     INTEGER NOT NULL DEFAULT 0,
    last_match_at   TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_screened_ip_addresses ON screened_ip_addresses(ip_address);

CREATE TABLE screened_urls (
    id              BIGSERIAL PRIMARY KEY,
    url             VARCHAR(1000) NOT NULL,
    domain          VARCHAR(255) NOT NULL,
    action_type     SMALLINT NOT NULL DEFAULT 0,
    match_count     INTEGER NOT NULL DEFAULT 0,
    last_match_at   TIMESTAMPTZ,
    ip_address      INET,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Watched words
CREATE TABLE watched_words (
    id              BIGSERIAL PRIMARY KEY,
    word            VARCHAR(1000) NOT NULL,
    action          SMALLINT NOT NULL,
    replacement     VARCHAR(1000),
    case_sensitive  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_watched_words_action ON watched_words(action);

CREATE TABLE watched_word_groups (
    id         BIGSERIAL PRIMARY KEY,
    action     SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 15. WEBHOOKS SUBSYSTEM
-- =============================================================================

CREATE TABLE web_hooks (
    id                    BIGSERIAL PRIMARY KEY,
    payload_url           VARCHAR(2000) NOT NULL,
    content_type          SMALLINT NOT NULL DEFAULT 1,
    last_delivery_status  SMALLINT NOT NULL DEFAULT 1,
    status                SMALLINT NOT NULL DEFAULT 1,
    secret                VARCHAR(255),
    wildcard_web_hook     BOOLEAN NOT NULL DEFAULT FALSE,
    verify_certificate    BOOLEAN NOT NULL DEFAULT TRUE,
    active                BOOLEAN NOT NULL DEFAULT FALSE,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE web_hook_event_types (
    id    BIGSERIAL PRIMARY KEY,
    name  VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE web_hook_event_types_hooks (
    web_hook_id            BIGINT NOT NULL REFERENCES web_hooks(id) ON DELETE CASCADE,
    web_hook_event_type_id BIGINT NOT NULL REFERENCES web_hook_event_types(id) ON DELETE CASCADE,
    PRIMARY KEY (web_hook_id, web_hook_event_type_id)
);

CREATE TABLE web_hook_events (
    id              BIGSERIAL PRIMARY KEY,
    web_hook_id     BIGINT NOT NULL REFERENCES web_hooks(id) ON DELETE CASCADE,
    headers         TEXT,
    payload         TEXT,
    status          SMALLINT,
    response_headers TEXT,
    response_body   TEXT,
    duration        INTEGER,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 16. API KEYS SUBSYSTEM
-- =============================================================================

CREATE TABLE api_keys (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT REFERENCES users(id) ON DELETE CASCADE,
    key_hash        VARCHAR(64) NOT NULL,
    description     TEXT,
    truncated_key   VARCHAR(4),
    allowed_ips     INET[],
    hidden          BOOLEAN NOT NULL DEFAULT FALSE,
    last_used_at    TIMESTAMPTZ,
    revoked_at      TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE api_key_scopes (
    id              BIGSERIAL PRIMARY KEY,
    api_key_id      BIGINT NOT NULL REFERENCES api_keys(id) ON DELETE CASCADE,
    resource        VARCHAR(255) NOT NULL,
    action          VARCHAR(255) NOT NULL,
    allowed_parameters JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 17. SITE SETTINGS SUBSYSTEM
-- =============================================================================

CREATE TABLE site_settings (
    id         BIGSERIAL PRIMARY KEY,
    name       VARCHAR(255) NOT NULL UNIQUE,
    data_type  SMALLINT NOT NULL,
    value      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 18. THEMES SUBSYSTEM
-- =============================================================================

CREATE TABLE themes (
    id                  BIGSERIAL PRIMARY KEY,
    name                VARCHAR(255) NOT NULL,
    user_id             BIGINT NOT NULL REFERENCES users(id),
    compiler_version    INTEGER NOT NULL DEFAULT 0,
    user_selectable     BOOLEAN NOT NULL DEFAULT FALSE,
    hidden              BOOLEAN NOT NULL DEFAULT FALSE,
    color_scheme_id     BIGINT,
    remote_theme_id     BIGINT,
    component           BOOLEAN NOT NULL DEFAULT FALSE,
    enabled             BOOLEAN NOT NULL DEFAULT TRUE,
    auto_update         BOOLEAN NOT NULL DEFAULT TRUE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE theme_fields (
    id                  BIGSERIAL PRIMARY KEY,
    theme_id            BIGINT NOT NULL REFERENCES themes(id) ON DELETE CASCADE,
    target_id           SMALLINT NOT NULL,
    name                VARCHAR(255) NOT NULL,
    value               TEXT,
    value_baked         TEXT,
    type_id             SMALLINT NOT NULL DEFAULT 0,
    compiler_version    INTEGER NOT NULL DEFAULT 0,
    error               TEXT,
    upload_id           BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(theme_id, target_id, name)
);

CREATE TABLE theme_settings (
    id          BIGSERIAL PRIMARY KEY,
    theme_id    BIGINT NOT NULL REFERENCES themes(id) ON DELETE CASCADE,
    name        VARCHAR(255) NOT NULL,
    data_type   SMALLINT NOT NULL,
    value       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE theme_modifier_sets (
    id                      BIGSERIAL PRIMARY KEY,
    theme_id                BIGINT NOT NULL UNIQUE REFERENCES themes(id) ON DELETE CASCADE,
    serialize_topic_excerpts BOOLEAN,
    csp_extensions          TEXT[],
    svg_icons               TEXT[],
    custom_homepage         BOOLEAN
);

CREATE TABLE child_themes (
    id              BIGSERIAL PRIMARY KEY,
    parent_theme_id BIGINT NOT NULL REFERENCES themes(id) ON DELETE CASCADE,
    child_theme_id  BIGINT NOT NULL REFERENCES themes(id) ON DELETE CASCADE,
    UNIQUE(parent_theme_id, child_theme_id)
);

CREATE TABLE remote_themes (
    id                      BIGSERIAL PRIMARY KEY,
    remote_url              VARCHAR(2000) NOT NULL,
    remote_version          VARCHAR(255),
    local_version           VARCHAR(255),
    commits_behind          INTEGER,
    branch                  VARCHAR(255),
    about_url               VARCHAR(2000),
    license_url             VARCHAR(2000),
    authors                 TEXT,
    theme_version           VARCHAR(255),
    minimum_discourse_version VARCHAR(255),
    maximum_discourse_version VARCHAR(255),
    last_error_text         TEXT,
    private_key             TEXT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 19. COLOR SCHEMES SUBSYSTEM
-- =============================================================================

CREATE TABLE color_schemes (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,
    version         INTEGER NOT NULL DEFAULT 1,
    via_wizard      BOOLEAN NOT NULL DEFAULT FALSE,
    base_scheme_id  VARCHAR(255),
    theme_id        BIGINT REFERENCES themes(id) ON DELETE SET NULL,
    user_selectable BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE color_scheme_colors (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,
    hex             VARCHAR(6) NOT NULL,
    color_scheme_id BIGINT NOT NULL REFERENCES color_schemes(id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 20. CHAT SUBSYSTEM
-- =============================================================================

CREATE TABLE chat_channels (
    id                   BIGSERIAL PRIMARY KEY,
    chatable_type        VARCHAR(100) NOT NULL,
    chatable_id          BIGINT NOT NULL,
    name                 VARCHAR(255),
    description          TEXT,
    slug                 VARCHAR(255),
    status               SMALLINT NOT NULL DEFAULT 0,
    user_count           INTEGER NOT NULL DEFAULT 0,
    last_message_id      BIGINT,
    auto_join_users      BOOLEAN NOT NULL DEFAULT FALSE,
    allow_channel_wide_mentions BOOLEAN NOT NULL DEFAULT TRUE,
    user_count_stale     BOOLEAN NOT NULL DEFAULT FALSE,
    messages_count       INTEGER NOT NULL DEFAULT 0,
    threading_enabled    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_chat_channels_chatable ON chat_channels(chatable_type, chatable_id);

CREATE TABLE chat_messages (
    id                  BIGSERIAL PRIMARY KEY,
    chat_channel_id     BIGINT NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    user_id             BIGINT NOT NULL REFERENCES users(id),
    message             TEXT,
    cooked              TEXT,
    cooked_version      SMALLINT,
    deleted_at          TIMESTAMPTZ,
    deleted_by_id       BIGINT REFERENCES users(id),
    in_reply_to_id      BIGINT REFERENCES chat_messages(id) ON DELETE SET NULL,
    last_editor_id      BIGINT REFERENCES users(id),
    excerpt             TEXT,
    thread_id           BIGINT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_chat_messages_channel ON chat_messages(chat_channel_id, created_at);

CREATE TABLE chat_threads (
    id                  BIGSERIAL PRIMARY KEY,
    channel_id          BIGINT NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    original_message_id BIGINT NOT NULL,
    original_message_user_id BIGINT NOT NULL REFERENCES users(id),
    status              SMALLINT NOT NULL DEFAULT 0,
    replies_count       INTEGER NOT NULL DEFAULT 0,
    title               VARCHAR(255),
    last_message_id     BIGINT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_chat_channel_memberships (
    id                      BIGSERIAL PRIMARY KEY,
    user_id                 BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    chat_channel_id         BIGINT NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    following               BOOLEAN NOT NULL DEFAULT TRUE,
    muted                   BOOLEAN NOT NULL DEFAULT FALSE,
    desktop_notification_level SMALLINT NOT NULL DEFAULT 1,
    mobile_notification_level  SMALLINT NOT NULL DEFAULT 1,
    last_read_message_id    BIGINT,
    last_unread_mention_when_emailed_id BIGINT,
    join_mode               SMALLINT NOT NULL DEFAULT 0,
    last_viewed_at          TIMESTAMPTZ,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, chat_channel_id)
);

CREATE TABLE user_chat_thread_memberships (
    id                      BIGSERIAL PRIMARY KEY,
    user_id                 BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    thread_id               BIGINT NOT NULL REFERENCES chat_threads(id) ON DELETE CASCADE,
    notification_level      SMALLINT NOT NULL DEFAULT 2,
    last_read_message_id    BIGINT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, thread_id)
);

CREATE TABLE chat_mentions (
    id              BIGSERIAL PRIMARY KEY,
    chat_message_id BIGINT NOT NULL REFERENCES chat_messages(id) ON DELETE CASCADE,
    target_type     VARCHAR(100) NOT NULL,
    target_id       BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_message_reactions (
    id              BIGSERIAL PRIMARY KEY,
    chat_message_id BIGINT NOT NULL REFERENCES chat_messages(id) ON DELETE CASCADE,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    emoji           VARCHAR(100) NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(chat_message_id, user_id, emoji)
);

CREATE TABLE chat_message_revisions (
    id              BIGSERIAL PRIMARY KEY,
    chat_message_id BIGINT NOT NULL REFERENCES chat_messages(id) ON DELETE CASCADE,
    old_message     TEXT NOT NULL,
    new_message     TEXT NOT NULL,
    user_id         BIGINT NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_uploads (
    id              BIGSERIAL PRIMARY KEY,
    chat_message_id BIGINT NOT NULL REFERENCES chat_messages(id) ON DELETE CASCADE,
    upload_id       BIGINT NOT NULL REFERENCES uploads(id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE chat_drafts (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    chat_channel_id BIGINT NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    data            TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, chat_channel_id)
);

CREATE TABLE direct_message_channels (
    id         BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE direct_message_users (
    id                          BIGSERIAL PRIMARY KEY,
    direct_message_channel_id   BIGINT NOT NULL REFERENCES direct_message_channels(id) ON DELETE CASCADE,
    user_id                     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(direct_message_channel_id, user_id)
);

-- =============================================================================
-- 21. POLLS SUBSYSTEM
-- =============================================================================

CREATE TABLE polls (
    id              BIGSERIAL PRIMARY KEY,
    post_id         BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    name            VARCHAR(255) NOT NULL DEFAULT 'poll',
    close_at        TIMESTAMPTZ,
    type            SMALLINT NOT NULL DEFAULT 0,
    status          SMALLINT NOT NULL DEFAULT 0,
    results         SMALLINT NOT NULL DEFAULT 0,
    min             INTEGER,
    max             INTEGER,
    step            INTEGER,
    anonymous_voters INTEGER,
    visibility      SMALLINT NOT NULL DEFAULT 0,
    chart_type      SMALLINT NOT NULL DEFAULT 0,
    groups          VARCHAR(255),
    title           VARCHAR(255),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, name)
);

CREATE TABLE poll_options (
    id          BIGSERIAL PRIMARY KEY,
    poll_id     BIGINT NOT NULL REFERENCES polls(id) ON DELETE CASCADE,
    digest      VARCHAR(40) NOT NULL,
    html        TEXT NOT NULL,
    anonymous_votes INTEGER,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE poll_votes (
    id              BIGSERIAL PRIMARY KEY,
    poll_id         BIGINT NOT NULL REFERENCES polls(id) ON DELETE CASCADE,
    poll_option_id  BIGINT NOT NULL REFERENCES poll_options(id) ON DELETE CASCADE,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(poll_id, poll_option_id, user_id)
);

-- =============================================================================
-- 22. PLUGIN INFRASTRUCTURE
-- =============================================================================

CREATE TABLE plugin_store_rows (
    id          BIGSERIAL PRIMARY KEY,
    plugin_name VARCHAR(255) NOT NULL,
    key         VARCHAR(255) NOT NULL,
    type_name   VARCHAR(255) NOT NULL,
    value       TEXT,
    UNIQUE(plugin_name, key)
);

CREATE TABLE user_exports (
    id              BIGSERIAL PRIMARY KEY,
    file_name       VARCHAR(255) NOT NULL,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    upload_id       BIGINT REFERENCES uploads(id) ON DELETE SET NULL,
    topic_id        BIGINT REFERENCES topics(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 23. SIDEBAR SUBSYSTEM
-- =============================================================================

CREATE TABLE sidebar_sections (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT REFERENCES users(id) ON DELETE CASCADE,
    title           VARCHAR(255) NOT NULL,
    public          BOOLEAN NOT NULL DEFAULT FALSE,
    section_type    SMALLINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sidebar_section_links (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    linkable_type       VARCHAR(100) NOT NULL,
    linkable_id         BIGINT NOT NULL,
    sidebar_section_id  BIGINT REFERENCES sidebar_sections(id) ON DELETE CASCADE,
    "position"          INTEGER NOT NULL DEFAULT 0,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sidebar_urls (
    id              BIGSERIAL PRIMARY KEY,
    name            VARCHAR(255) NOT NULL,
    value           VARCHAR(1000) NOT NULL,
    icon            VARCHAR(100) NOT NULL DEFAULT 'link',
    external        BOOLEAN NOT NULL DEFAULT FALSE,
    segment         SMALLINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 24. FORM TEMPLATES
-- =============================================================================

CREATE TABLE form_templates (
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL UNIQUE,
    template    TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE category_form_templates (
    id              BIGSERIAL PRIMARY KEY,
    category_id     BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    form_template_id BIGINT NOT NULL REFERENCES form_templates(id) ON DELETE CASCADE,
    "order"         INTEGER NOT NULL DEFAULT 0,
    UNIQUE(category_id, form_template_id)
);

-- =============================================================================
-- 25. EMBEDDABLE HOSTS
-- =============================================================================

CREATE TABLE embeddable_hosts (
    id              BIGSERIAL PRIMARY KEY,
    host            VARCHAR(255) NOT NULL,
    category_id     BIGINT REFERENCES categories(id) ON DELETE SET NULL,
    class_name      VARCHAR(255),
    allowed_paths   VARCHAR(1024),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 26. PERMALINKS
-- =============================================================================

CREATE TABLE permalinks (
    id              BIGSERIAL PRIMARY KEY,
    url             VARCHAR(1000) NOT NULL UNIQUE,
    topic_id        BIGINT REFERENCES topics(id) ON DELETE SET NULL,
    post_id         BIGINT REFERENCES posts(id) ON DELETE SET NULL,
    category_id     BIGINT REFERENCES categories(id) ON DELETE SET NULL,
    tag_id          BIGINT REFERENCES tags(id) ON DELETE SET NULL,
    external_url    VARCHAR(1000),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- 27. SEARCH LOGS
-- =============================================================================

CREATE TABLE search_logs (
    id              BIGSERIAL PRIMARY KEY,
    term            VARCHAR(1000) NOT NULL,
    user_id         BIGINT REFERENCES users(id) ON DELETE SET NULL,
    ip_address      INET,
    search_result_id BIGINT,
    search_type     SMALLINT NOT NULL DEFAULT 0,
    search_result_type SMALLINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_search_logs_term ON search_logs(term);
CREATE INDEX idx_search_logs_created_at ON search_logs(created_at);

-- =============================================================================
-- 28. BACKUPS
-- =============================================================================

CREATE TABLE backups (
    id              BIGSERIAL PRIMARY KEY,
    filename        VARCHAR(255) NOT NULL,
    size            BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE backup_metadata (
    id          BIGSERIAL PRIMARY KEY,
    backup_id   BIGINT NOT NULL REFERENCES backups(id) ON DELETE CASCADE,
    name        VARCHAR(255) NOT NULL,
    value       TEXT
);

-- =============================================================================
-- 29. JOBS / SCHEDULER
-- =============================================================================

CREATE TABLE scheduler_stats (
    id          BIGSERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    hostname    VARCHAR(255),
    pid         INTEGER,
    started_at  TIMESTAMPTZ NOT NULL,
    live_slots_start INTEGER,
    live_slots_finish INTEGER,
    success     BOOLEAN,
    error       TEXT,
    duration_ms INTEGER
);

CREATE TABLE given_daily_likes (
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    likes_given INTEGER NOT NULL,
    given_date  DATE NOT NULL,
    limit_reached BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (user_id, given_date)
);

-- =============================================================================
-- 30. MISCELLANEOUS / INFRASTRUCTURE TABLES
-- =============================================================================

CREATE TABLE stylesheet_cache (
    id          BIGSERIAL PRIMARY KEY,
    target      VARCHAR(255) NOT NULL,
    digest      VARCHAR(40) NOT NULL,
    content     TEXT NOT NULL,
    theme_id    BIGINT REFERENCES themes(id) ON DELETE CASCADE,
    source_map  TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE translation_overrides (
    id              BIGSERIAL PRIMARY KEY,
    locale          VARCHAR(10) NOT NULL,
    translation_key VARCHAR(500) NOT NULL,
    value           TEXT NOT NULL,
    compiled_js     TEXT,
    original_translation TEXT,
    status          SMALLINT NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(locale, translation_key)
);

CREATE TABLE application_requests (
    id          BIGSERIAL PRIMARY KEY,
    date        DATE NOT NULL,
    req_type    SMALLINT NOT NULL,
    count       INTEGER NOT NULL DEFAULT 0,
    UNIQUE(date, req_type)
);

CREATE TABLE user_badge_removals (
    id              BIGSERIAL PRIMARY KEY,
    user_badge_id   BIGINT NOT NULL,
    badge_id        BIGINT NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    removed_by_id   BIGINT REFERENCES users(id),
    seq             INTEGER NOT NULL DEFAULT 0,
    post_id         BIGINT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE tag_search_data (
    tag_id      BIGINT PRIMARY KEY REFERENCES tags(id) ON DELETE CASCADE,
    search_data TSVECTOR,
    raw_data    TEXT,
    locale      VARCHAR(10)
);
CREATE INDEX idx_tag_search_data ON tag_search_data USING GIN(search_data);

CREATE TABLE topic_hot_scores (
    id          BIGSERIAL PRIMARY KEY,
    topic_id    BIGINT NOT NULL UNIQUE REFERENCES topics(id) ON DELETE CASCADE,
    score       REAL NOT NULL DEFAULT 0,
    recent_likes INTEGER NOT NULL DEFAULT 0,
    recent_posters INTEGER NOT NULL DEFAULT 0,
    recent_first_bumped_at TIMESTAMPTZ,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE category_permission_sets (
    id              BIGSERIAL PRIMARY KEY,
    category_id     BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    permission_type SMALLINT NOT NULL,
    group_id        BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE
);

CREATE TABLE published_pages (
    id              BIGSERIAL PRIMARY KEY,
    topic_id        BIGINT NOT NULL UNIQUE REFERENCES topics(id) ON DELETE CASCADE,
    slug            VARCHAR(255) NOT NULL UNIQUE,
    public          BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE shared_drafts (
    id          BIGSERIAL PRIMARY KEY,
    topic_id    BIGINT NOT NULL UNIQUE REFERENCES topics(id) ON DELETE CASCADE,
    category_id BIGINT NOT NULL REFERENCES categories(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_warnings (
    id              BIGSERIAL PRIMARY KEY,
    topic_id        BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_by_id   BIGINT NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_ip_address_histories (
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    ip_address INET NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE topic_link_counts (
    id              BIGSERIAL PRIMARY KEY,
    topic_link_id   BIGINT NOT NULL REFERENCES topic_links(id) ON DELETE CASCADE,
    user_id         BIGINT REFERENCES users(id) ON DELETE SET NULL,
    ip_address      INET,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_security_keys (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_id   TEXT NOT NULL UNIQUE,
    public_key      TEXT NOT NULL,
    factor_type     SMALLINT NOT NULL DEFAULT 0,
    enabled         BOOLEAN NOT NULL DEFAULT TRUE,
    name            VARCHAR(255) NOT NULL,
    last_used       TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE category_featured_users (
    id          BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE post_stats (
    id              BIGSERIAL PRIMARY KEY,
    post_id         BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    drafts_saved    INTEGER NOT NULL DEFAULT 0,
    typing_duration_msecs INTEGER NOT NULL DEFAULT 0,
    composer_open_duration_msecs INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE badge_posts (
    id              BIGSERIAL PRIMARY KEY,
    badge_id        BIGINT NOT NULL REFERENCES badges(id) ON DELETE CASCADE,
    post_id         BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    UNIQUE(badge_id, post_id)
);

CREATE TABLE group_archived_messages (
    id          BIGSERIAL PRIMARY KEY,
    group_id    BIGINT NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    topic_id    BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE user_archived_messages (
    id          BIGSERIAL PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_id    BIGINT NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE allowed_pm_users (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    allowed_pm_user_id  BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, allowed_pm_user_id)
);

-- =============================================================================
-- 31. DEFERRED FOREIGN KEYS
-- Add FK constraints that reference tables created later in the script
-- =============================================================================

-- users -> groups (primary_group_id, flair_group_id)
ALTER TABLE users ADD CONSTRAINT fk_users_primary_group
    FOREIGN KEY (primary_group_id) REFERENCES groups(id) ON DELETE SET NULL;
ALTER TABLE users ADD CONSTRAINT fk_users_flair_group
    FOREIGN KEY (flair_group_id) REFERENCES groups(id) ON DELETE SET NULL;

-- users -> uploads (uploaded_avatar_id)
ALTER TABLE users ADD CONSTRAINT fk_users_uploaded_avatar
    FOREIGN KEY (uploaded_avatar_id) REFERENCES uploads(id) ON DELETE SET NULL;

-- categories -> topics (topic_id, latest_topic_id)
ALTER TABLE categories ADD CONSTRAINT fk_categories_topic
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE SET NULL;
ALTER TABLE categories ADD CONSTRAINT fk_categories_latest_topic
    FOREIGN KEY (latest_topic_id) REFERENCES topics(id) ON DELETE SET NULL;
ALTER TABLE categories ADD CONSTRAINT fk_categories_latest_post
    FOREIGN KEY (latest_post_id) REFERENCES posts(id) ON DELETE SET NULL;

-- category_featured_topics -> topics
ALTER TABLE category_featured_topics ADD CONSTRAINT fk_cft_topic
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE;

-- category_tag_stats -> categories, tags
ALTER TABLE category_tag_stats ADD CONSTRAINT fk_cts_category
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE;
ALTER TABLE category_tag_stats ADD CONSTRAINT fk_cts_tag
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE;

-- topic_links -> posts
ALTER TABLE topic_links ADD CONSTRAINT fk_topic_links_post
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE;
ALTER TABLE topic_links ADD CONSTRAINT fk_topic_links_link_post
    FOREIGN KEY (link_post_id) REFERENCES posts(id) ON DELETE SET NULL;

-- user_actions -> topics, posts
ALTER TABLE user_actions ADD CONSTRAINT fk_user_actions_target_topic
    FOREIGN KEY (target_topic_id) REFERENCES topics(id) ON DELETE SET NULL;
ALTER TABLE user_actions ADD CONSTRAINT fk_user_actions_target_post
    FOREIGN KEY (target_post_id) REFERENCES posts(id) ON DELETE SET NULL;

-- user_badges -> posts
ALTER TABLE user_badges ADD CONSTRAINT fk_user_badges_post
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE SET NULL;
ALTER TABLE user_badges ADD CONSTRAINT fk_user_badges_notification
    FOREIGN KEY (notification_id) REFERENCES notifications(id) ON DELETE SET NULL;

-- user_histories -> topics, posts, categories
ALTER TABLE user_histories ADD CONSTRAINT fk_user_histories_topic
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE SET NULL;
ALTER TABLE user_histories ADD CONSTRAINT fk_user_histories_post
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE SET NULL;
ALTER TABLE user_histories ADD CONSTRAINT fk_user_histories_category
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL;

-- user_profiles -> uploads
ALTER TABLE user_profiles ADD CONSTRAINT fk_user_profiles_bg
    FOREIGN KEY (profile_background_upload_id) REFERENCES uploads(id) ON DELETE SET NULL;
ALTER TABLE user_profiles ADD CONSTRAINT fk_user_profiles_card_bg
    FOREIGN KEY (card_background_upload_id) REFERENCES uploads(id) ON DELETE SET NULL;

-- user_avatars -> uploads
ALTER TABLE user_avatars ADD CONSTRAINT fk_user_avatars_custom
    FOREIGN KEY (custom_upload_id) REFERENCES uploads(id) ON DELETE SET NULL;
ALTER TABLE user_avatars ADD CONSTRAINT fk_user_avatars_gravatar
    FOREIGN KEY (gravatar_upload_id) REFERENCES uploads(id) ON DELETE SET NULL;

-- email_change_requests -> email_tokens
ALTER TABLE email_change_requests ADD CONSTRAINT fk_ecr_old_token
    FOREIGN KEY (old_email_token_id) REFERENCES email_tokens(id) ON DELETE SET NULL;
ALTER TABLE email_change_requests ADD CONSTRAINT fk_ecr_new_token
    FOREIGN KEY (new_email_token_id) REFERENCES email_tokens(id) ON DELETE SET NULL;

-- uploads -> posts (access_control_post_id)
ALTER TABLE uploads ADD CONSTRAINT fk_uploads_access_control_post
    FOREIGN KEY (access_control_post_id) REFERENCES posts(id) ON DELETE SET NULL;

-- themes -> color_schemes, remote_themes
ALTER TABLE themes ADD CONSTRAINT fk_themes_color_scheme
    FOREIGN KEY (color_scheme_id) REFERENCES color_schemes(id) ON DELETE SET NULL;
ALTER TABLE themes ADD CONSTRAINT fk_themes_remote_theme
    FOREIGN KEY (remote_theme_id) REFERENCES remote_themes(id) ON DELETE SET NULL;

-- chat_threads -> chat_messages (original_message_id, last_message_id)
ALTER TABLE chat_threads ADD CONSTRAINT fk_chat_threads_original_message
    FOREIGN KEY (original_message_id) REFERENCES chat_messages(id) ON DELETE CASCADE;
ALTER TABLE chat_threads ADD CONSTRAINT fk_chat_threads_last_message
    FOREIGN KEY (last_message_id) REFERENCES chat_messages(id) ON DELETE SET NULL;

-- chat_messages -> chat_threads
ALTER TABLE chat_messages ADD CONSTRAINT fk_chat_messages_thread
    FOREIGN KEY (thread_id) REFERENCES chat_threads(id) ON DELETE SET NULL;

-- chat_channels -> chat_messages (last_message_id)
ALTER TABLE chat_channels ADD CONSTRAINT fk_chat_channels_last_message
    FOREIGN KEY (last_message_id) REFERENCES chat_messages(id) ON DELETE SET NULL;

-- topic_embeds -> posts
ALTER TABLE topic_embeds ADD CONSTRAINT fk_topic_embeds_post
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE SET NULL;

COMMIT;
