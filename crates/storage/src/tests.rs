#[test]
fn test_create_file() -> crate::Result<()> {
  use std::path::Path;

  const STORAGE: &'static str = "storage-test-test_create_file";
  if let Err(_) = std::fs::remove_dir_all(STORAGE) {}
  if let Err(_) = crate::initialize(STORAGE, Some(2)) {};

  let one = dbg!(crate::write("one.md", "content one", ())?);
  let two = crate::write("two.text", "content two", ())?;
  let three = crate::write("three.rs", "content three", ())?;

  let (_, path) = crate::read(&three)?;
  println!("path: {path:?}");

  let content = std::fs::read_to_string(path)?;

  // verify both one & two have the same bucket
  let one = Path::new(&one).parent().unwrap();
  let two = Path::new(&two).parent().unwrap();
  assert_eq!(one, two);

  let three = Path::new(&three).parent().unwrap();
  assert_ne!(two, three);

  assert_eq!(content, "content three");

  if let Err(_) = std::fs::remove_dir_all(STORAGE) {}

  Ok(())
}

#[test]
fn test_create_metadata_file() -> crate::Result<()> {
  use serde::Deserialize;
  use serde::Serialize;

  #[derive(Serialize, Deserialize)]
  struct TestMetadata {
    alias: String,
  }

  const STORAGE: &'static str = "storage-test-test_create_metadata_file";
  if let Err(_) = std::fs::remove_dir_all(STORAGE) {}
  if let Err(_) = crate::initialize(STORAGE, Some(2)) {};

  let one = crate::write(
    "one.md",
    "content one",
    Some(TestMetadata {
      alias: "an-alias.md".to_owned(),
    }),
  )?;

  let metadata: Option<TestMetadata> = crate::deserialize_metadata(&one)?;

  assert!(metadata.is_some());

  let metadata = metadata.unwrap();
  assert_eq!(metadata.alias, "an-alias.md");

  if let Err(_) = std::fs::remove_dir_all(STORAGE) {}

  Ok(())
}
