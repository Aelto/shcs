use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub alias: String,
    pub custom: Option<serde_json::Value>,
}
