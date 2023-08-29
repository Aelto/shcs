use crate::*;

pub struct Item;

impl Item {
  pub fn path(root: &std::path::PathBuf, bucket: &str, name: &str) -> std::path::PathBuf {
    Bucket::path(root, bucket).join(name)
  }

  pub fn file(
    root: &std::path::PathBuf, bucket: &str, name: &str,
  ) -> Result<(std::fs::File, std::path::PathBuf)> {
    let path = Self::path(root, bucket, name);
    let file = std::fs::File::open(&path)?;

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

  pub fn persist_tempfile(
    root: &std::path::PathBuf, bucket: &str, name: &str, tempfile: tempfile::NamedTempFile,
  ) -> Result<()> {
    tempfile
      .persist(Self::path(root, bucket, name))
      .map_err(|e| e.error)?; // IoError

    Ok(())
  }

  pub fn to_string(bucket: &str, name: &str) -> String {
    format!("{bucket}/{name}")
  }
}
