use regex::Regex;

pub fn process_hashtags(html: &str) -> String {
    let re = Regex::new(r"#([a-zA-Z0-9_-]+)").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let name = &caps[1];
        format!(r#"<a class="hashtag" href="/tag/{name}">#{name}</a>"#)
    }).to_string()
}
