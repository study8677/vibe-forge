pub fn process_images(html: &str) -> String {
    // Add loading="lazy" to all images
    html.replace("<img ", r#"<img loading="lazy" "#)
}
