pub fn preprocess_details(raw: &str) -> String {
    raw.replace("[details=", "<details><summary>")
       .replace("]", "</summary>")
       .replace("[/details]", "</details>")
}
