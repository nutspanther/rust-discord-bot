use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResult {
    pub etag: String,
    pub items: Vec<SearchResultItem>,
    pub kind: String,
    pub next_page_token: String,
    pub page_info: Value,
    pub region_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultItem {
    pub etag: String,
    pub id: SearchResultItemId,
    pub kind: String,
    pub snippet: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultItemId {
    pub kind: String,
    pub video_id: String,
}
