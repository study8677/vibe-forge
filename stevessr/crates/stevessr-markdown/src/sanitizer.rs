pub struct Sanitizer {
    cleaner: ammonia::Builder<'static>,
}

impl Sanitizer {
    pub fn new() -> Self {
        let mut builder = ammonia::Builder::default();
        builder
            .add_tags(&["details", "summary", "abbr", "ruby", "rt", "rp", "mark"])
            .add_tag_attributes("div", &["class", "data-theme", "data-controller"])
            .add_tag_attributes("span", &["class"])
            .add_tag_attributes("a", &["class", "data-username", "data-group"])
            .add_tag_attributes("img", &["class", "loading", "width", "height", "alt", "title"])
            .add_tag_attributes("code", &["class"])
            .add_tag_attributes("pre", &["class", "data-code-wrap"])
            .add_tag_attributes("aside", &["class"])
            .add_tag_attributes("blockquote", &["class"]);

        Self { cleaner: builder }
    }

    pub fn sanitize(&self, html: &str) -> String {
        self.cleaner.clean(html).to_string()
    }
}

impl Default for Sanitizer {
    fn default() -> Self {
        Self::new()
    }
}
