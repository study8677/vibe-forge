use regex::Regex;

pub fn process_emoji(html: &str) -> String {
    let re = Regex::new(r":([a-zA-Z0-9_+-]+):").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let name = &caps[1];
        format!(r#"<img class="emoji" title=":{name}:" src="/images/emoji/{name}.png" alt=":{name}:">"#)
    }).to_string()
}
