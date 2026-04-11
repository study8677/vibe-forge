-- =============================================================================
-- stevessr: 0002_seed_data.sql
-- Seed initial reference data required for the forum to function
-- =============================================================================

BEGIN;

-- =============================================================================
-- 1. BADGE TYPES (Gold, Silver, Bronze)
-- =============================================================================

INSERT INTO badge_types (id, name) VALUES
    (1, 'Gold'),
    (2, 'Silver'),
    (3, 'Bronze');

SELECT setval('badge_types_id_seq', 3);

-- =============================================================================
-- 2. BADGE GROUPINGS
-- =============================================================================

INSERT INTO badge_groupings (id, name, "position") VALUES
    (1, 'Getting Started', 10),
    (2, 'Community',       11),
    (3, 'Posting',         12),
    (4, 'Trust Level',     13);

SELECT setval('badge_groupings_id_seq', 4);

-- =============================================================================
-- 3. DEFAULT SYSTEM BADGES
-- =============================================================================

INSERT INTO badges (id, name, description, badge_type_id, badge_grouping_id, multiple_grant, system, enabled, listable, icon) VALUES
    -- Trust Level badges (grouping 4)
    (1,  'Basic User',     'Granted upon reaching trust level 1',                3, 4, FALSE, TRUE, TRUE, TRUE, 'fa-user'),
    (2,  'Member',         'Granted upon reaching trust level 2',                3, 4, FALSE, TRUE, TRUE, TRUE, 'fa-user'),
    (3,  'Regular',        'Granted upon reaching trust level 3',                2, 4, FALSE, TRUE, TRUE, TRUE, 'fa-user'),
    (4,  'Leader',         'Granted upon reaching trust level 4',                1, 4, FALSE, TRUE, TRUE, TRUE, 'fa-user'),

    -- Getting Started badges (grouping 1)
    (5,  'Welcome',        'Received a like on a post',                          3, 1, FALSE, TRUE, TRUE, TRUE, 'fa-handshake'),
    (6,  'Nice Topic',     'Received 10 likes on a topic',                       3, 1, FALSE, TRUE, TRUE, TRUE, 'fa-commenting'),
    (7,  'Good Topic',     'Received 25 likes on a topic',                       2, 1, FALSE, TRUE, TRUE, TRUE, 'fa-commenting'),
    (8,  'Great Topic',    'Received 50 likes on a topic',                       1, 1, FALSE, TRUE, TRUE, TRUE, 'fa-commenting'),
    (9,  'Autobiographer', 'Filled out profile bio and profile picture',         3, 1, FALSE, TRUE, TRUE, TRUE, 'fa-address-card'),
    (10, 'Editor',         'Edited a post',                                       3, 1, FALSE, TRUE, TRUE, TRUE, 'fa-pencil-alt'),
    (11, 'Reader',         'Read every reply in a topic with more than 100 replies', 3, 1, FALSE, TRUE, TRUE, TRUE, 'fa-book-reader'),
    (12, 'Read Guidelines','Read the community guidelines',                       3, 1, FALSE, TRUE, TRUE, TRUE, 'fa-book'),

    -- Posting badges (grouping 3)
    (13, 'Nice Post',      'Received 10 likes on a post',                        3, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-heart'),
    (14, 'Good Post',      'Received 25 likes on a post',                        2, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-heart'),
    (15, 'Great Post',     'Received 50 likes on a post',                        1, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-heart'),
    (16, 'Nice Reply',     'Received 10 likes on a reply',                       3, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-reply'),
    (17, 'Good Reply',     'Received 25 likes on a reply',                       2, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-reply'),
    (18, 'Great Reply',    'Received 50 likes on a reply',                       1, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-reply'),
    (19, 'Popular Link',   'Shared a link that was clicked by 50 outside users', 2, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-link'),
    (20, 'Hot Link',       'Shared a link that was clicked by 300 outside users',1, 3, TRUE,  TRUE, TRUE, TRUE, 'fa-link'),
    (21, 'First Like',     'Liked a post for the first time',                    3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-heart'),
    (22, 'First Share',    'Shared a post for the first time',                   3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-share'),
    (23, 'First Flag',     'Flagged a post for the first time',                  3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-flag'),
    (24, 'First Emoji Reaction', 'Used an emoji reaction for the first time',    3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-smile'),
    (25, 'First Quote',    'Quoted a post for the first time',                   3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-quote-left'),
    (26, 'First Mention',  'Mentioned a user for the first time',               3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-at'),
    (27, 'First Reply By Email', 'Replied to a post via email',                  3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-envelope'),
    (28, 'First Onebox',   'Posted a link that was oneboxed',                    3, 3, FALSE, TRUE, TRUE, TRUE, 'fa-external-link-alt'),

    -- Community badges (grouping 2)
    (29, 'Empathetic',     'Received 500 likes across all posts',                2, 2, FALSE, TRUE, TRUE, TRUE, 'fa-hands-helping'),
    (30, 'Appreciated',    'Received 1000 likes across all posts',               1, 2, FALSE, TRUE, TRUE, TRUE, 'fa-trophy'),
    (31, 'Thank You',      'Received 20 likes on a single post',                 3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-thumbs-up'),
    (32, 'Gives Back',     'Gave 100 likes',                                     2, 2, FALSE, TRUE, TRUE, TRUE, 'fa-gift'),
    (33, 'Devotee',        'Visited the site 365 consecutive days',              2, 2, FALSE, TRUE, TRUE, TRUE, 'fa-calendar-check'),
    (34, 'Aficionado',     'Visited the site 100 consecutive days',              3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-calendar'),
    (35, 'Enthusiast',     'Visited the site 10 consecutive days',               3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-star'),
    (36, 'Anniversary',    'Active member for a year',                           2, 2, TRUE,  TRUE, TRUE, TRUE, 'fa-birthday-cake'),
    (37, 'Licensed',       'Completed all new user onboarding tips',             3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-id-card'),
    (38, 'Certified',      'Completed the advanced user tutorial',               3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-certificate'),
    (39, 'Campaign',       'Invited 3 users who became Basic Users',             3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-paper-plane'),
    (40, 'Promoter',       'Invited a user who became a Basic User',             3, 2, FALSE, TRUE, TRUE, TRUE, 'fa-user-plus'),
    (41, 'Champion',       'Invited 5 users who became Members',                 1, 2, FALSE, TRUE, TRUE, TRUE, 'fa-award');

SELECT setval('badges_id_seq', 41);

-- =============================================================================
-- 4. DEFAULT GROUPS
-- =============================================================================

INSERT INTO groups (id, name, automatic, visibility_level, user_count) VALUES
    (0,  'everyone',       TRUE, 0, 0),
    (1,  'admins',         TRUE, 1, 0),
    (2,  'moderators',     TRUE, 1, 0),
    (3,  'staff',          TRUE, 1, 0),
    (10, 'trust_level_0',  TRUE, 0, 0),
    (11, 'trust_level_1',  TRUE, 0, 0),
    (12, 'trust_level_2',  TRUE, 0, 0),
    (13, 'trust_level_3',  TRUE, 0, 0),
    (14, 'trust_level_4',  TRUE, 0, 0);

SELECT setval('groups_id_seq', 14);

-- =============================================================================
-- 5. POST ACTION TYPES
-- =============================================================================

INSERT INTO post_action_types (id, name_key, is_flag, icon, "position", score_bonus, reviewable_priority) VALUES
    (1, 'bookmark',            FALSE, 'bookmark',      1,  0,   0),
    (2, 'like',                FALSE, 'heart',         2,  0,   0),
    (3, 'off_topic',           TRUE,  'far-hand',      3,  4.0, 5),
    (4, 'inappropriate',       TRUE,  'flag',          4,  4.0, 10),
    (5, 'vote',                FALSE, 'far-thumbs-up', 5,  0,   0),
    (6, 'spam',                TRUE,  'far-eye-slash', 7,  4.0, 10),
    (7, 'notify_user',         TRUE,  'far-envelope',  8,  0,   0),
    (8, 'notify_moderators',   TRUE,  'far-flag',      9,  4.0, 10);

SELECT setval('post_action_types_id_seq', 8);

-- =============================================================================
-- 6. WEBHOOK EVENT TYPES
-- =============================================================================

INSERT INTO web_hook_event_types (id, name) VALUES
    (1,  'topic'),
    (2,  'post'),
    (3,  'user'),
    (4,  'group'),
    (5,  'category'),
    (6,  'tag'),
    (7,  'notification'),
    (8,  'reviewable'),
    (9,  'chat_message'),
    (10, 'flag'),
    (11, 'like'),
    (12, 'user_badge'),
    (13, 'solved'),
    (14, 'assign'),
    (15, 'user_promoted');

SELECT setval('web_hook_event_types_id_seq', 15);

-- =============================================================================
-- 7. ESSENTIAL SITE SETTINGS
-- =============================================================================

INSERT INTO site_settings (name, data_type, value) VALUES
    -- Basic site identity (data_type 1 = string, 5 = integer, 7 = bool)
    ('title',                           1, 'stevessr Forum'),
    ('site_description',                1, 'A community discussion platform'),
    ('contact_email',                   1, ''),
    ('contact_url',                     1, ''),
    ('notification_email',              1, 'noreply@example.com'),
    ('logo_url',                        1, ''),
    ('logo_small_url',                  1, ''),
    ('digest_logo_url',                 1, ''),
    ('mobile_logo_url',                 1, ''),
    ('large_icon_url',                  1, ''),
    ('favicon_url',                     1, ''),
    ('apple_touch_icon_url',            1, ''),
    ('default_opengraph_image_url',     1, ''),

    -- Locale & regional
    ('default_locale',                  1, 'en'),
    ('allow_user_locale',               7, 'true'),
    ('set_locale_from_accept_language_header', 7, 'false'),

    -- Login & authentication
    ('login_required',                  7, 'false'),
    ('must_approve_users',              7, 'false'),
    ('enable_local_logins',             7, 'true'),
    ('enable_local_logins_via_email',   7, 'true'),
    ('allow_new_registrations',         7, 'true'),
    ('enable_google_oauth2_logins',     7, 'false'),
    ('enable_github_logins',            7, 'false'),
    ('enable_twitter_logins',           7, 'false'),
    ('enable_facebook_logins',          7, 'false'),
    ('enable_discord_logins',           7, 'false'),
    ('enable_sso',                      7, 'false'),
    ('sso_url',                         1, ''),
    ('sso_secret',                      1, ''),
    ('sso_overrides_email',             7, 'false'),
    ('sso_overrides_username',          7, 'false'),
    ('sso_overrides_name',              7, 'false'),
    ('sso_overrides_avatar',            7, 'false'),
    ('invite_only',                     7, 'false'),
    ('min_password_length',             5, '10'),
    ('min_admin_password_length',       5, '15'),
    ('enable_2fa',                      7, 'true'),
    ('enforce_2fa',                     1, 'no'),

    -- Trust levels
    ('default_trust_level',             5, '0'),
    ('tl1_requires_topics_entered',     5, '5'),
    ('tl1_requires_read_posts',         5, '30'),
    ('tl1_requires_time_spent_mins',    5, '10'),
    ('tl2_requires_topics_entered',     5, '20'),
    ('tl2_requires_read_posts',         5, '100'),
    ('tl2_requires_time_spent_mins',    5, '60'),
    ('tl2_requires_days_visited',       5, '15'),
    ('tl2_requires_likes_received',     5, '1'),
    ('tl2_requires_likes_given',        5, '1'),
    ('tl2_requires_topic_reply_count',  5, '3'),
    ('tl3_requires_days_visited',       5, '50'),
    ('tl3_requires_topics_replied_to',  5, '10'),
    ('tl3_requires_topics_viewed',      5, '25'),
    ('tl3_requires_posts_read',         5, '25'),
    ('tl3_requires_topics_viewed_all_time', 5, '200'),
    ('tl3_requires_posts_read_all_time',    5, '500'),
    ('tl3_requires_max_flagged',        5, '5'),
    ('tl3_promotion_min_duration',      5, '50'),

    -- Posting
    ('min_post_length',                 5, '20'),
    ('min_first_post_length',           5, '20'),
    ('min_personal_message_post_length',5, '10'),
    ('max_post_length',                 5, '32000'),
    ('topic_featured_link_enabled',     7, 'true'),
    ('min_topic_title_length',          5, '15'),
    ('max_topic_title_length',          5, '255'),
    ('allow_duplicate_topic_titles',    7, 'false'),
    ('min_personal_message_title_length', 5, '2'),
    ('editing_grace_period',            5, '300'),
    ('editing_grace_period_max_diff',   5, '100'),
    ('post_edit_time_limit',            5, '525600'),
    ('newuser_max_links',               5, '2'),
    ('newuser_max_images',              5, '1'),
    ('newuser_max_attachments',         5, '0'),
    ('newuser_max_mentions_per_post',   5, '2'),
    ('newuser_max_replies_per_topic',   5, '3'),
    ('max_mentions_per_post',           5, '10'),
    ('max_tags_per_topic',              5, '5'),
    ('allow_uncategorized_topics',      7, 'true'),
    ('suppress_uncategorized_badge',    7, 'true'),
    ('default_categories_watching',     1, ''),
    ('default_categories_tracking',     1, ''),
    ('default_categories_muted',        1, ''),

    -- Rate limits
    ('rate_limit_create_topic',         5, '15'),
    ('rate_limit_create_post',          5, '5'),
    ('rate_limit_new_user_create_topic',5, '120'),
    ('rate_limit_new_user_create_post', 5, '30'),
    ('max_topics_per_day',              5, '20'),
    ('max_personal_messages_per_day',   5, '20'),
    ('max_likes_per_day',               5, '50'),
    ('max_bookmarks_per_day',           5, '20'),
    ('max_edits_per_day',               5, '30'),
    ('max_invites_per_day',             5, '10'),
    ('max_flags_per_day',               5, '20'),

    -- Email
    ('email_time_window_mins',          5, '10'),
    ('email_posts_context',             5, '5'),
    ('digest_min_days',                 5, '7'),
    ('disable_digest_emails',           7, 'false'),
    ('default_email_digest_frequency',  5, '10080'),
    ('default_email_level',             5, '1'),
    ('default_email_messages_level',    5, '0'),

    -- Uploads
    ('max_attachment_size_kb',          5, '4096'),
    ('max_image_size_kb',               5, '4096'),
    ('max_image_width',                 5, '690'),
    ('max_image_height',                5, '500'),
    ('authorized_extensions',           1, 'jpg|jpeg|png|gif|svg|txt|pdf|zip|tar.gz|mp4|webm|mp3|ogg|wav'),
    ('allow_uploaded_avatars',          7, 'true'),
    ('default_avatars',                 1, ''),
    ('allow_animated_avatars',          7, 'false'),

    -- Moderation
    ('num_flaggers_to_close_topic',     5, '5'),
    ('auto_respond_to_flag_actions',    7, 'true'),
    ('min_flags_staff_visibility',      5, '1'),
    ('score_required_to_hide_post',     5, '10'),
    ('cooldown_minutes_after_hiding_posts', 5, '10'),
    ('num_users_to_block_new_user',     5, '3'),
    ('notify_mods_when_user_silenced',  7, 'false'),
    ('flag_sockpuppets',                7, 'false'),
    ('approve_unless_trust_level',      5, '0'),

    -- Chat
    ('chat_enabled',                    7, 'true'),
    ('chat_allowed_groups',             1, '0'),
    ('chat_max_message_length',         5, '6000'),
    ('chat_allow_uploads',              7, 'true'),
    ('direct_message_enabled_groups',   1, '0'),

    -- Plugins / misc
    ('poll_enabled',                    7, 'true'),
    ('poll_minimum_trust_level_to_create', 5, '1'),
    ('tagging_enabled',                 7, 'true'),
    ('max_tag_length',                  5, '20'),
    ('force_https',                     7, 'true'),
    ('enable_backups',                  7, 'true'),
    ('backup_frequency',                5, '7'),

    -- Performance & caching
    ('anonymous_cache_duration',        5, '60'),
    ('anon_polling_interval',           5, '25000'),
    ('polling_interval',                5, '3000'),
    ('background_polling_interval',     5, '60000'),

    -- Search
    ('log_search_queries',              7, 'true'),
    ('min_search_term_length',          5, '3'),
    ('search_prefer_recent_posts',      7, 'true'),
    ('search_recent_posts_size',        5, '100000');

COMMIT;
