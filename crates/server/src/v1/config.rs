use super::ApiError;

#[derive(Debug, serde::Deserialize, Default)]
pub struct Config {
  enabled: bool,

  /// The server sends a POST request to the supplied endpoint to authenticate
  /// the operations.
  authentication_endpoint: String,

  /// The server sends a POST request to the supplied endpoint to notify of the
  /// completed operation.
  completion_endpoint: String,
}

impl Config {
  /// Obtain a version of the configuration file from the disk
  pub fn from_disk() -> Result<Self, ApiError> {
    let content = std::fs::read_to_string("v1.shcs.toml")?;

    Ok(toml::from_str(&content)?)
  }

  pub fn enabled() -> Result<bool, ApiError> {
    Self::from_disk().map(|c| c.enabled)
  }

  pub fn authentication_endpoint(&self) -> &str {
    &self.authentication_endpoint
  }

  pub fn completion_endpoint(&self) -> &str {
    &self.completion_endpoint
  }
}
