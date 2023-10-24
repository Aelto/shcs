//! Functions to send HTTP request to the API endpoints supported by the v1 API
use super::UrlBuilder;

#[derive(Debug)]
pub enum Error {
  Serde(serde_json::Error),
  Reqwest(reqwest::Error),
  UnhandledStatus(reqwest::StatusCode),
  InvalidUrl,
}

impl From<serde_json::Error> for Error {
  fn from(value: serde_json::Error) -> Self {
    Self::Serde(value)
  }
}

impl From<reqwest::Error> for Error {
  fn from(value: reqwest::Error) -> Self {
    Self::Reqwest(value)
  }
}

/// Upload the file and get the storage path in return
pub async fn upload_file(
  domain: &str, authorization: String, file: Vec<u8>, filename: Option<String>,
  metadata: Option<impl serde::Serialize>,
) -> Result<String, Error> {
  let url = UrlBuilder::new(domain).ok()?;
  let mut filepart = reqwest::multipart::Part::stream(file);

  if let Some(filename) = filename {
    filepart = filepart.file_name(filename);
  }

  let mut form = reqwest::multipart::Form::new().part("file", filepart);

  if let Some(metadata) = metadata {
    let json = serde_json::to_string(&metadata)?;
    let metadatapart = reqwest::multipart::Part::text(json).mime_str("application/json")?;

    form = form.part("metadata", metadatapart);
  }

  let response = reqwest::Client::new()
    .put(url)
    .multipart(form)
    .header("Authorization", authorization)
    .send()
    .await?;

  let status = response.status();
  match status {
    reqwest::StatusCode::CREATED => Ok(response.text().await?),
    _ => Err(Error::UnhandledStatus(status)),
  }
}

/// This function is lower level than the other **C**R**UD** functions as it
/// returns a raw `Response` object directly. This allows the response to be used
/// in proxy functions
pub async fn get_file(
  client: &reqwest::Client, domain: &str, bucket: &str, item: &str,
) -> Result<reqwest::Response, Error> {
  let storage_path = storage::internal::storage_path(bucket, item);
  let url = UrlBuilder::new(domain).join(&storage_path).ok()?;
  let req = client.get(url);
  let res = req.send().await?;

  let status = res.status();
  match status {
    reqwest::StatusCode::OK => Ok(res),
    _ => Err(Error::UnhandledStatus(status)),
  }
}

pub async fn delete_file(
  domain: &str, authorization: String, bucket: &str, item: &str,
) -> Result<(), Error> {
  let storage_path = storage::internal::storage_path(bucket, item);
  let url = UrlBuilder::new(domain).join(&storage_path).ok()?;

  let response = reqwest::Client::new()
    .delete(url)
    .header("Authorization", authorization)
    .send()
    .await?;

  let status = response.status();
  match status {
    reqwest::StatusCode::OK => Ok(()),
    _ => Err(Error::UnhandledStatus(status)),
  }
}
