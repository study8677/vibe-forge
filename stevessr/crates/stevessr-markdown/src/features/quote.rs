use regex::Regex;

pub fn process_quotes(html: &str) -> String {
    let re = Regex::new(r#"\[quote="([^"]+)"\]([\s\S]*?)\[/quote\]"#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let attribution = &caps[1];
        let content = &caps[2];
        format!(
            r#"<aside class="quote" data-username="{attribution}"><div class="title">{attribution}:</div><blockquote>{content}</blockquote></aside>"#
        )
    }).to_string()
}
