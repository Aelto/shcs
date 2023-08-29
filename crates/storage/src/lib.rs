mod bucket;
pub(crate) use bucket::*;

mod item;
pub(crate) use item::*;

mod metadata;
pub(crate) use metadata::*;

mod storage;

mod error;
pub use error::*;

mod config;
pub(crate) use config::*;

mod dotfile;
pub(crate) use dotfile::*;

pub(crate) mod constants;

#[cfg(test)]
mod tests;

pub use crate::config::initialize;
pub use crate::storage::deserialize_metadata;
pub use crate::storage::exists;
pub use crate::storage::persist_tempfile;
pub use crate::storage::read;
pub use crate::storage::read_metadata;
pub use crate::storage::remove;
pub use crate::storage::replace_tempfile;
pub use crate::storage::write;

pub use crate::storage::internal::bucket_and_item;
pub use crate::storage::internal::set_metadata;
pub use crate::storage::internal::storage_path;
pub use crate::storage::internal::write_exact;
