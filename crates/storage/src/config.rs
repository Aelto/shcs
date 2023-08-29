use crate::*;

pub(crate) struct Config {
  pub(crate) root: std::path::PathBuf,
  active_bucket_name: std::sync::RwLock<String>,

  /// the maximum size of a bucket
  bucket_size: usize,
}

static CONFIG: once_cell::sync::OnceCell<Config> = once_cell::sync::OnceCell::new();

/// Initialize the storage system to use the given `root` directory for its
/// internal storage. Then optionally set the maximum number of items a single
/// bucket can hold. If set to `None` then it will use the default optimized
/// value.
///
/// ```rs
/// storage::initialize(".", None).await?;
/// ```
/// or
/// ```rs
/// storage::initialize(".", Some(10)).await?;
/// ```
pub fn initialize(
  root: impl Into<std::path::PathBuf>, custom_bucket_size: Option<usize>,
) -> Result<()> {
  let root = root.into();
  let dotfile = DotFile::from_file(&root)?;
  let active_bucket_name = dotfile.active_bucket_name.into_owned();

  if let Err(_) = std::fs::create_dir_all(Bucket::path(&root, &active_bucket_name)) {}

  CONFIG
    .set(Config {
      root: root,
      active_bucket_name: active_bucket_name.into(),
      bucket_size: custom_bucket_size.unwrap_or(constants::BUCKET_SIZE_MAX),
    })
    .map_err(|_| StorageError::ConfigAlreadySet)
}

pub(crate) fn config() -> Result<&'static Config> {
  CONFIG.get().ok_or(StorageError::ConfigNotSet)
}

impl Config {
  pub(crate) fn with_bucket<'a>(&self) -> Result<String> {
    let active_bucket_size = {
      let name = self.active_bucket_name.read()?;

      Bucket::size(&self.root, &name)?
    };

    if active_bucket_size >= self.bucket_size {
      let mut active_bucket = self.active_bucket_name.write()?;
      let mut new_bucket_name = None;

      // this loop ensures the newly created bucket doesn't point to an already
      // existing one.
      while let None = new_bucket_name {
        let bucket_name = Bucket::new_random_name();

        if !Bucket::exists(&self.root, &bucket_name) {
          new_bucket_name = Some(bucket_name);
        }
      }

      let new_bucket_name = new_bucket_name.unwrap_or_default();

      std::fs::create_dir_all(&Bucket::path(&self.root, &new_bucket_name))?;

      DotFile {
        active_bucket_name: std::borrow::Cow::from(&new_bucket_name),
      }
      .to_file(&self.root)?;

      // dotfile.active_bucket_name(&self.root, bucket_name).await?;
      *active_bucket = new_bucket_name;
    }

    Ok(self.active_bucket_name.read()?.clone())
  }
}
