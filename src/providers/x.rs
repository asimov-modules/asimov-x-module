// This is free and unencumbered software released into the public domain.

use crate::api::types::XType;
use crate::providers::provider::Provider;

pub static X_LIST_MEMBERS: Provider = Provider {
    id: XType::LIST_MEMBERS_ID,
    url: "https://x.com/i/lists",
};

pub static URL_PREFIX_TO_PROVIDER: [(&str, &'static Provider); 1] = [
    ("https://x.com/i/lists", &X_LIST_MEMBERS),
];

pub fn extract_list_id_from_url(url: &str) -> Option<String> {
    // Extract ID from URLs like: https://x.com/i/lists/random_string
    if let Some(stripped) = url.strip_prefix("https://x.com/i/lists/") {
        if !stripped.is_empty() && stripped.chars().all(|c| c.is_ascii_digit()) {
            return Some(stripped.to_string());
        }
    }
    None
}
