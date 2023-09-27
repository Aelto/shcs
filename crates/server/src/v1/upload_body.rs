use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;

#[derive(Debug, MultipartForm)]
pub struct UploadFileBody {
  pub metadata: Option<actix_multipart::form::json::Json<serde_json::Value>>,
  pub file: TempFile,
}

impl UploadFileBody {
  pub fn into_metadata(
    self, bucket: &str,
  ) -> Result<(super::Metadata, String, TempFile), super::ApiError> {
    let user_filename = self.file.file_name.clone();
    let unique_id = Self::next_unique_id(bucket)?;

    let mut filename = unique_id.clone();
    if let Some(name) = user_filename.as_ref() {
      let user_extension = std::path::Path::new(&name).extension();
      if let Some(ext) = user_extension {
        if let Some(ext) = ext.to_str() {
          filename.push('.');
          filename.push_str(ext);
        }
      }
    }

    let metadata = super::Metadata {
      alias: user_filename.unwrap_or_else(|| unique_id),
      custom: self.metadata.map(|j| j.0),
    };

    Ok((metadata, filename, self.file))
  }

  fn next_unique_id(bucket: &str) -> Result<String, super::ApiError> {
    for _ in 0..100 {
      let id = nanoid::nanoid!();
      let storage_path = storage::internal::storage_path(bucket, &id);

      if !storage::exists(&storage_path)? {
        return Ok(id);
      }
    }

    Err(super::ApiError::InternalServerError)
  }
}
