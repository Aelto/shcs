pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Debug)]
pub enum StorageError {
  ConfigNotSet,
  ConfigAlreadySet,
  Io(std::io::Error),
  Serde(serde_yaml::Error),
  PoisonError,
  Custom(&'static str),

  ReadMissingBucket,
  ReadMissingItem,
}

impl From<std::io::Error> for StorageError {
  fn from(value: std::io::Error) -> Self {
    Self::Io(value)
  }
}

impl From<serde_yaml::Error> for StorageError {
  fn from(value: serde_yaml::Error) -> Self {
    Self::Serde(value)
  }
}

impl<T> From<std::sync::PoisonError<T>> for StorageError {
  fn from(_: std::sync::PoisonError<T>) -> Self {
    Self::PoisonError
  }
}

impl std::fmt::Display for StorageError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StorageError::ConfigNotSet => write!(f, "config not set"),
      StorageError::ConfigAlreadySet => write!(f, "config already set"),
      StorageError::Io(io) => write!(f, "io error: {io}"),
      StorageError::Serde(e) => write!(f, "serde error: {e}"),
      StorageError::PoisonError => write!(f, "rwlock poison error"),
      StorageError::ReadMissingBucket => write!(f, "read failure, missing bucket name"),
      StorageError::ReadMissingItem => write!(f, "read failure, missing item name"),
      StorageError::Custom(s) => write!(f, "{s}"),
    }
  }
}

impl std::error::Error for StorageError {}
