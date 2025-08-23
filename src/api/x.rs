// This is free and unencumbered software released into the public domain.

use crate::api::types::XListMembersResponse;
use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct XConfig {
    pub base_url: String,
    pub access_token: String,
}

impl XConfig {
    pub fn new() -> Result<Self> {
        let access_token = env::var("X_TOKEN")
            .map_err(|_| anyhow::anyhow!("X_TOKEN environment variable not set"))?;

        Ok(Self {
            base_url: "https://api.x.com/2".to_string(),
            access_token,
        })
    }

    pub fn endpoint_url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

pub struct XClient {
    config: XConfig,
}

impl XClient {
    pub fn new() -> Result<Self> {
        let config = XConfig::new()?;
        Ok(Self { config })
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.config.access_token)
    }

    pub fn fetch_list_members(
        &self,
        list_id: &str,
        limit: Option<usize>,
    ) -> Result<XListMembersResponse> { 
        if limit.is_none() {
            return self.fetch_all_list_members(list_id);
        }
        
        self.fetch_list_members_page(list_id, limit, None)
    }

    fn fetch_all_list_members(&self, list_id: &str) -> Result<XListMembersResponse> {
        let mut all_users = Vec::new();
        let mut next_token: Option<String> = None;

        loop {
            let response = self.fetch_list_members_page(list_id, None, next_token.as_deref())?;
            
            if let Some(users) = response.data {
                all_users.extend(users);
            }

            next_token = response.meta.and_then(|meta| meta.next_token);
            
            if next_token.is_none() {
                break;
            }
        }

        Ok(XListMembersResponse {
            data: Some(all_users),
            meta: None,
        })
    }

    fn fetch_list_members_page(
        &self,
        list_id: &str,
        limit: Option<usize>,
        pagination_token: Option<&str>,
    ) -> Result<XListMembersResponse> {
        let mut url = format!(
            "{}/lists/{}/members?user.fields=id,name,username,description,location,profile_image_url,profile_banner_url,verified,protected,created_at,public_metrics",
            self.config.base_url, list_id
        );

        if let Some(limit_val) = limit {
            url.push_str(&format!("&max_results={}", limit_val));
        }

        if let Some(token) = pagination_token {
            url.push_str(&format!("&pagination_token={}", token));
        }

        let mut response = ureq::get(&url)
            .header("Authorization", &self.auth_header())
            .call()
            .map_err(|e| {
                if e.to_string().contains("429") {
                    anyhow::anyhow!("Rate limit exceeded (429). Please wait before trying again.")
                } else if e.to_string().contains("401") {
                    anyhow::anyhow!("Unauthorized (401). Please check your X_TOKEN.")
                } else {
                    e.into()
                }
            })?;

        let response_body: XListMembersResponse =
            serde_json::from_str(&response.body_mut().read_to_string()?)?;
        Ok(response_body)
    }
}
