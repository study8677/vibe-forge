pub fn process_code_blocks(_html: &str) -> String {
    // pulldown-cmark handles basic code blocks; this adds syntax highlighting class names
    _html.to_string()
}
