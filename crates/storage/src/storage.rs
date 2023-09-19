use crate::*;

/// Read the file at given `storage_path` and return the File & its path.
///
/// A storage path consists of a string, split by a `/` where the left side
/// is the bucket_name and the right side is the filename: `qsdo34-23d/filename.md`
///
/// ```rs
/// storage::read("qsdo34-23d/filename.md")
/// ```
///
/// If the supplied path does not point to an existing file, then an error is
/// returned.
pub fn read(storage_path: &str) -> Result<(std::fs::File, std::path::PathBuf)> {
  let (bucket, item) = internal::bucket_and_item(storage_path)?;

  Item::file(&config()?.root, bucket, item)
}

/// Read the metadata for the file sitting at the given `storage_path`.
///
/// A storage path consists of a string, split by a `/` where the left side
/// is the bucket_name and the right side is the filename: `qsdo34-23d/filename.md`
///
/// ```rs
/// storage::read("qsdo34-23d/filename.md")
/// ```
///
/// Due to the optional nature of the metadata file, and unlike the [read()]
/// function, if the supplied path does not point to an existing file, then None
/// is returned.
pub fn read_metadata(storage_path: &str) -> Result<(Option<std::fs::File>, std::path::PathBuf)> {
  let (bucket, item) = internal::bucket_and_item(storage_path)?;

  Metadata::file(&config()?.root, bucket, item)
}

/// Read the metadata for the file sitting at the given `storage_path` and
/// deserialize the content into the returned value `M` as long as `M` implements
/// [serde::Deserialize]
pub fn deserialize_metadata<M>(storage_path: &str) -> Result<Option<M>>
where
  M: serde::de::DeserializeOwned,
{
  let (file, path) = read_metadata(storage_path)?;

  let metadata = match file.is_some() {
    true => {
      let content = std::fs::read_to_string(path)?;

      Some(serde_yaml::from_str(&content)?)
    }
    false => None,
  };

  Ok(metadata)
}

/// Returns whether the given path points to an existing item.
///
/// A storage path consists of a string, split by a `/` where the left side
/// is the bucket_name and the right side is the filename: `qsdo34-23d/filename.md`
///
/// ```rs
/// storage::exists("qsdo34-23d/filename.md")
/// ```
pub fn exists(storage_path: &str) -> Result<bool> {
  let (bucket, item) = internal::bucket_and_item(storage_path)?;

  Ok(Item::exists(&config()?.root, bucket, item))
}

/// Remove the file sitting at `storage_path` while also removing the optional
/// metadata file that is linked to the file.
pub fn remove(storage_path: &str) -> Result<()> {
  let (bucket, item) = internal::bucket_and_item(storage_path)?;
  let root = &config()?.root;

  let item_removal = Item::remove(root, bucket, item);
  let mut metadata_removal = Ok(());

  if Metadata::exists(&root, bucket, item) {
    metadata_removal = Metadata::remove(root, bucket, item);
  }

  item_removal.and(metadata_removal)
}

pub fn write<M>(name: &str, content: &str, metadata: M) -> Result<String>
where
  M: serde::Serialize,
{
  let config = config()?;
  let active_bucket = config.with_bucket()?;

  let storage_path = internal::write_exact(&config.root, &active_bucket, name, content)?;
  internal::set_metadata(&storage_path, metadata)?;

  Ok(storage_path)
}

pub fn persist_tempfile<M>(
  name: &str, tempfile: tempfile::NamedTempFile, metadata: M,
) -> Result<String>
where
  M: serde::Serialize,
{
  let config = config()?;
  let active_bucket = config.with_bucket()?;

  Item::persist_tempfile(&config.root, &active_bucket, &name, tempfile)?;

  let storage_path = internal::storage_path(&active_bucket, name);
  internal::set_metadata(&storage_path, metadata)?;

  Ok(storage_path)
}

pub fn replace_tempfile<M>(
  storage_path: &str, tempfile: tempfile::NamedTempFile, metadata: M,
) -> Result<String>
where
  M: serde::Serialize,
{
  let (active_bucket, name) = internal::bucket_and_item(storage_path)?;

  Item::persist_tempfile(&config()?.root, &active_bucket, &name, tempfile)?;

  let storage_path = internal::storage_path(&active_bucket, name);
  internal::set_metadata(&storage_path, metadata)?;

  Ok(storage_path)
}

/// Internal functions that can be used to precisely control the storage system &
/// circumvent some of the automatic behaviours.
pub mod internal {
  use std::path::PathBuf;

  use super::Item;
  use super::Metadata;
  use super::Result;
  use super::StorageError;

  use super::config;

  /// Get the name of the currently active bucket
  pub fn active_bucket<'a>() -> Result<String> {
    config()?.with_bucket()
  }

  /// Parses and returns the `bucket` and the `item` name from the provided
  /// `storage_path`
  pub fn bucket_and_item(storage_path: &str) -> Result<(&str, &str)> {
    let mut split = storage_path.split("/");
    let (bucket, item) = (
      split.next().ok_or(StorageError::ReadMissingBucket)?,
      split.next().ok_or(StorageError::ReadMissingItem)?,
    );

    Ok((bucket, item))
  }

  /// Generates the storage path from the `bucket` and the `item`
  pub fn storage_path(bucket: &str, name: &str) -> String {
    Item::to_string(bucket, name)
  }

  /// Forcefully write an `item` inside the provided `bucket`
  pub fn write_exact(root: &PathBuf, bucket: &str, item: &str, content: &str) -> Result<String> {
    Item::write(root, bucket, item, content)?;

    Ok(storage_path(bucket, item))
  }

  pub fn set_metadata<M>(storage_path: &str, metadata: M) -> Result<()>
  where
    M: serde::Serialize,
  {
    let (bucket, item) = bucket_and_item(storage_path)?;

    // There is no point in creating an empty metadata file
    if std::mem::size_of::<M>() > 0 {
      let content = serde_yaml::to_string(&metadata)?;
      Metadata::write(&config()?.root, bucket, item, &content)?;
    };

    Ok(())
  }
}
