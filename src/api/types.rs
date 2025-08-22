// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub count: Option<u32>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Option<Vec<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightRequest {
    pub text: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub source_url: Option<String>,
    pub source_type: Option<String>,
    pub category: Option<String>,
    pub note: Option<String>,
    pub location: Option<i32>,
    pub location_type: Option<String>,
    pub highlighted_at: Option<String>,
    pub highlight_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Highlight {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub source: Option<String>,
    pub num_highlights: Option<u32>,
    pub last_highlight_at: Option<String>,
    pub updated: Option<String>,
    pub cover_image_url: Option<String>,
    pub highlights_url: Option<String>,
    pub source_url: Option<String>,
    pub modified_highlights: Option<Vec<u64>>,
    pub text: Option<String>,
    pub source_type: Option<String>,
    pub note: Option<String>,
    pub location: Option<i32>,
    pub location_type: Option<String>,
    pub highlighted_at: Option<String>,
    pub highlight_url: Option<String>,
}

pub type HighlightsResponse = PaginatedResponse<Highlight>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub source: Option<String>,
    pub num_highlights: Option<u32>,
    pub last_highlight_at: Option<String>,
    pub updated: Option<String>,
    pub cover_image_url: Option<String>,
    pub highlights_url: Option<String>,
    pub source_url: Option<String>,
    pub asin: Option<String>,
    pub tags: Option<Vec<String>>,
    pub document_note: Option<String>,
}

pub type BookListResponse = PaginatedResponse<Book>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub type TagsResponse = PaginatedResponse<Tag>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleTag {
    pub name: Option<String>,
    pub updated: Option<i64>,
    pub count: Option<u32>,
}

pub type SimpleTagsResponse = Vec<SimpleTag>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Highlights(HighlightsResponse),
    BookList(BookListResponse),
    Tags(TagsResponse),
    SimpleTags(SimpleTagsResponse),
}

pub use Book as BookResponse;
pub use Highlight as HighlightResponse;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XType {
    ListMembers,
}

impl XType {
    pub const LIST_MEMBERS_ID: &'static str = "x-list-members";

    pub fn as_str(&self) -> &'static str {
        match self {
            XType::ListMembers => Self::LIST_MEMBERS_ID,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUser {
    pub id: String,
    pub name: String,
    pub username: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub profile_image_url: Option<String>,
    pub profile_banner_url: Option<String>,
    pub verified: Option<bool>,
    pub protected: Option<bool>,
    pub created_at: Option<String>,
    pub public_metrics: Option<XUserMetrics>,
    pub entities: Option<XUserEntities>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUserMetrics {
    pub followers_count: Option<i32>,
    pub following_count: Option<i32>,
    pub tweet_count: Option<i32>,
    pub listed_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUserEntities {
    pub url: Option<XUrlEntity>,
    pub description: Option<XDescriptionEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUrlEntity {
    pub urls: Option<Vec<XUrlInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XDescriptionEntity {
    pub urls: Option<Vec<XUrlInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XUrlInfo {
    pub url: String,
    pub expanded_url: String,
    pub display_url: String,
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XListMembersResponse {
    pub data: Option<Vec<XUser>>,
    pub meta: Option<XListMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XListMeta {
    pub result_count: Option<i32>,
    pub next_token: Option<String>,
    pub previous_token: Option<String>,
}


