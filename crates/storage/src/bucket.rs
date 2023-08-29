use crate::*;

#[derive(Clone)]
pub struct Bucket;

impl Bucket {
  /// Generate a new random bucket name
  pub(crate) fn new_random_name() -> String {
    let id = nanoid::nanoid!();

    id
  }

  pub(crate) fn path(root: &std::path::PathBuf, name: &str) -> std::path::PathBuf {
    root.join(name)
  }

  pub(crate) fn exists(root: &std::path::PathBuf, name: &str) -> bool {
    Self::path(root, name).exists()
  }

  /// Returns the number of files in the bucket
  pub(crate) fn size(root: &std::path::PathBuf, name: &str) -> Result<usize> {
    let count = std::fs::read_dir(Self::path(root, name))?.count();

    Ok(count)
  }
}
