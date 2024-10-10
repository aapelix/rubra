use url::Url;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DOMAIN_PATTERN: Regex = Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z]{2,})+$").unwrap();
}

pub fn process_search_input(input: &str) -> String {
    if let Ok(url) = Url::parse(input) {
        if url.scheme() != "" {
            return input.to_string();
        }

        if url.host_str().map_or(false, |host| host == "localhost" || host.starts_with("127.")) {
            return format!("http://{}", input);
        }

        if url.path().starts_with('/') {
            return format!("file://{}", input);
        }

        return format!("https://{}", input);
    }

    if DOMAIN_PATTERN.is_match(input) {
        return format!("https://{}", input);
    }

    let query = urlencoding::encode(input);
    format!("https://duckduckgo.com/?q={}", query)
}
