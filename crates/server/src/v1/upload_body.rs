use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;

#[derive(Debug, MultipartForm)]
pub struct UploadFileBody {
  pub metadata: Option<actix_multipart::form::json::Json<serde_json::Value>>,
  pub file: TempFile,
}

impl UploadFileBody {
  pub fn into_metadata(self) -> (super::Metadata, String, TempFile) {
    let user_filename = self.file.file_name.clone();
    let unique_id = nanoid::nanoid!();

    let metadata = super::Metadata {
      alias: user_filename.unwrap_or_else(|| unique_id.clone()),
      custom: self.metadata.map(|j| j.0),
    };

    (metadata, unique_id, self.file)
  }
}
