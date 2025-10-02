use serde::{Deserialize, Serialize};
use serde_json::Value;

/// LSP Message structure
#[derive(Debug, Serialize, Deserialize)]
pub struct LspMessage {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

/// Hover content structure
#[derive(Debug, Serialize, Deserialize)]
pub struct HoverContent {
    pub contents: HoverContents,
    pub range: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    Single(HoverContentItem),
    Multiple(Vec<HoverContentItem>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HoverContentItem {
    pub language: Option<String>,
    pub value: String,
    pub kind: Option<String>,
}
