// This is free and unencumbered software released into the public domain.

use asimov_x_module::{find_provider_for, providers::x::extract_list_id_from_url};

#[test]
fn test_find_provider_for_x_list_url() {
    let url = "https://x.com/i/lists/1958261568329924954";
    let provider = find_provider_for(url);
    assert!(provider.is_some());
    assert_eq!(provider.unwrap().id, "x-list-members");
}

#[test]
fn test_find_provider_for_unsupported_url() {
    let url = "https://example.com/api/books";
    let provider = find_provider_for(url);
    assert!(provider.is_none());
}

#[test]
fn test_extract_list_id_from_url() {
    let url = "https://x.com/i/lists/1958261568329924954";
    let id = extract_list_id_from_url(url);
    assert_eq!(id, Some("1958261568329924954".to_string()));
}

#[test]
fn test_extract_list_id_from_invalid_url() {
    let url = "https://x.com/i/lists/";
    let id = extract_list_id_from_url(url);
    assert_eq!(id, None);
}
