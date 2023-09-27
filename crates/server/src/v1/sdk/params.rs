use super::api::Error;
use reqwest::Url;

pub struct UrlBuilder(String);
impl UrlBuilder {
  pub(crate) fn new(base: &str) -> Self {
    Self(String::new()).join(base).join("v1")
  }

  pub(crate) fn join(mut self, segment: &str) -> Self {
    if !self.0.is_empty() {
      if !self.0.ends_with("/") && !segment.starts_with("/") {
        self.0.push('/');
      }
    }

    self.0.push_str(segment);
    self
  }

  pub(crate) fn ok(self) -> Result<Url, Error> {
    Url::parse(&self.0).map_err(|_| Error::InvalidUrl)
  }
}
