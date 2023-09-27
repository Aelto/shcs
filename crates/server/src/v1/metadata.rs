use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  pub alias: String,
  pub custom: Option<serde_json::Value>,
}
