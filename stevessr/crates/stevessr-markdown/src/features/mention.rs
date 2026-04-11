use regex::Regex;

pub fn process_mentions(html: &str) -> String {
    let re = Regex::new(r"@([a-zA-Z0-9_.-]+)").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let username = &caps[1];
        format!(r#"<a class="mention" href="/u/{username}">@{username}</a>"#)
    }).to_string()
}
