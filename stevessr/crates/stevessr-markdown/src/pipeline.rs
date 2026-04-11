use crate::features;
use crate::sanitizer::Sanitizer;

pub struct MarkdownPipeline {
    sanitizer: Sanitizer,
}

impl MarkdownPipeline {
    pub fn new() -> Self {
        Self {
            sanitizer: Sanitizer::new(),
        }
    }

    /// Convert raw markdown to cooked HTML.
    /// This is the main entry point, equivalent to Discourse's PrettyText.cook().
    pub fn cook(&self, raw: &str) -> String {
        // Phase 1: Pre-process raw markdown
        let preprocessed = self.preprocess(raw);

        // Phase 2: Parse markdown to HTML
        let html = self.parse_markdown(&preprocessed);

        // Phase 3: Post-process HTML (mentions, hashtags, oneboxes, etc.)
        let processed = self.postprocess(&html);

        // Phase 4: Sanitize
        self.sanitizer.sanitize(&processed)
    }

    fn preprocess(&self, raw: &str) -> String {
        let mut text = raw.to_string();
        text = features::poll::preprocess_polls(&text);
        text = features::spoiler::preprocess_spoilers(&text);
        text = features::details::preprocess_details(&text);
        text
    }

    fn parse_markdown(&self, raw: &str) -> String {
        use pulldown_cmark::{Parser, Options, html};
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(raw, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    fn postprocess(&self, html: &str) -> String {
        let mut result = html.to_string();
        result = features::mention::process_mentions(&result);
        result = features::hashtag::process_hashtags(&result);
        result = features::emoji::process_emoji(&result);
        result = features::quote::process_quotes(&result);
        result = features::image::process_images(&result);
        result
    }
}

impl Default for MarkdownPipeline {
    fn default() -> Self {
        Self::new()
    }
}
