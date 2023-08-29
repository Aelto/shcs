use crate::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct DotFile<'a> {
  pub(crate) active_bucket_name: std::borrow::Cow<'a, str>,
}

impl<'a> DotFile<'a> {
  fn path(root: &std::path::PathBuf) -> std::path::PathBuf {
    root.join(".storage")
  }

  /// Returns whether the dotfile exists or not
  fn exists(root: &std::path::PathBuf) -> bool {
    Self::path(root).exists()
  }

  pub fn from_file(root: &std::path::PathBuf) -> Result<DotFile<'a>> {
    let dotfile = match Self::exists(root) {
      true => {
        let content = std::fs::read_to_string(Self::path(root))?;
        serde_yaml::from_str(&content)?
      }
      false => {
        // if it doesn't exist, create a default one and write to the file
        let dotfile = Self::default();

        dotfile.to_file(root)?;

        dotfile
      }
    };

    Ok(dotfile)
  }

  pub(crate) fn to_file(&self, root: &std::path::PathBuf) -> Result<()> {
    if let Err(_) = std::fs::create_dir_all(root) {}

    std::fs::write(Self::path(root), serde_yaml::to_string(&self)?)?;

    Ok(())
  }
}

impl<'a> Default for DotFile<'a> {
  fn default() -> Self {
    Self {
      active_bucket_name: Bucket::new_random_name().into(),
    }
  }
}
