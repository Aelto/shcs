use std::fmt::Display;

/// Describes the attempt to perform an operation
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[repr(usize)]
pub enum Operation {
  Upload = 0,
  Replace = 1,
  ReplaceActive = 2,
  MetadataSet = 3,
  MetadataGet = 4,
  Delete = 5,
}

impl Display for Operation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Operation::Upload => write!(f, "Upload"),
      Operation::Replace => write!(f, "Replace"),
      Operation::ReplaceActive => write!(f, "ReplaceActive"),
      Operation::MetadataSet => write!(f, "MetadataSet"),
      Operation::MetadataGet => write!(f, "MetadataGet"),
      Operation::Delete => write!(f, "Delete"),
    }
  }
}
