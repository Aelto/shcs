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
