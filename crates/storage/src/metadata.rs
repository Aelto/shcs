use crate::*;

pub struct Metadata;

impl Metadata {
  fn metadata_filename(name: &str) -> String {
    format!("{name}.metadata.yaml")
  }

  pub fn path(root: &std::path::PathBuf, bucket: &str, name: &str) -> std::path::PathBuf {
    let item_path = Item::path(root, bucket, name);
    let item_filename = item_path
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();

    let metadata_filename = Self::metadata_filename(item_filename);

    item_path.with_file_name(metadata_filename)
  }

  pub fn file(
    root: &std::path::PathBuf, bucket: &str, name: &str,
  ) -> Result<(Option<std::fs::File>, std::path::PathBuf)> {
    let path = Self::path(root, bucket, name);

    let file = match Self::exists(root, bucket, name) {
      true => Some(std::fs::File::open(&path)?),
      false => None,
    };

    Ok((file, path))
  }

  pub fn write(root: &std::path::PathBuf, bucket: &str, name: &str, content: &str) -> Result<()> {
    std::fs::write(Self::path(root, bucket, name), content)?;

    Ok(())
  }

  pub fn exists(root: &std::path::PathBuf, bucket: &str, name: &str) -> bool {
    Self::path(root, bucket, name).exists()
  }

  pub fn remove(root: &std::path::PathBuf, bucket: &str, name: &str) -> Result<()> {
    std::fs::remove_file(Self::path(root, bucket, name))?;

    Ok(())
  }
}
