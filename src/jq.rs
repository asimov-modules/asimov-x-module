// This is free and unencumbered software released into the public domain.
pub use ::jq::*;

pub fn x_list() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/x_list.jq").parse().unwrap())
}
