use regex::Regex;

pub fn preprocess_spoilers(raw: &str) -> String {
    let re = Regex::new(r"\[spoiler\]([\s\S]*?)\[/spoiler\]").unwrap();
    re.replace_all(raw, |caps: &regex::Captures| {
        let content = &caps[1];
        format!(r#"<details class="spoiler"><summary>Spoiler</summary>{content}</details>"#)
    }).to_string()
}
