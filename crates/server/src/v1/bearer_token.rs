use std::future::Future;
use std::pin::Pin;

use actix_web::FromRequest;
use actix_web::HttpRequest;

use super::ApiError;
use super::Config;

/// A user extracted from the `Authorization` Bearer token
#[derive(Debug)]
pub struct BearerToken {
  authorization: reqwest::header::HeaderValue,
}

/// Holds the response from the successful authententication of the [BearerToken]
#[derive(Debug, Clone)]
pub struct AuthenticatedBearerIdentifier(String);

impl FromRequest for BearerToken {
  type Error = ApiError;
  type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

  fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
    let token_header = req.headers().get("Authorization").cloned();

    Box::pin(async move {
      token_header
        .ok_or(ApiError::Unauthorized)
        .and_then(|header| {
          header
            .to_str()
            .map_err(|_| ApiError::Unauthorized)
            .map(|s| s.to_owned())
        })
        .and_then(|s| s.parse().map_err(|_| ApiError::Unauthorized))
        .map(|s| Self { authorization: s })
    })
  }
}

impl BearerToken {
  /// Contact the configured authentication endpoint to validate the stored
  /// authorization header token.
  ///
  /// Any error or any response other than a 200: OK yields an UNAUTHORIZED
  /// error.
  pub async fn authenticate(
    &self, config: &Config, action: super::sdk::Operation,
  ) -> Result<AuthenticatedBearerIdentifier, ApiError> {
    match self.is_authorized(config, action).await {
      Ok(Some(identifier)) => Ok(identifier),
      _ => Err(ApiError::Unauthorized),
    }
  }

  async fn is_authorized(
    &self, config: &Config, action: super::sdk::Operation,
  ) -> Result<Option<AuthenticatedBearerIdentifier>, ApiError> {
    let url = config.authentication_endpoint();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("Authorization", self.authorization.clone());

    let body = serde_json::to_string(&action).map_err(|_| ApiError::InternalServerError)?;

    let client = reqwest::Client::new();
    let resp = client
      .post(url)
      .body(body)
      .headers(headers)
      .header("Content-Type", "application/json")
      .send()
      .await?;

    // any other status than 200 is considered to be UNAUTHORIZED
    let authorized = resp.status() == reqwest::StatusCode::OK;

    // expect the authentication endpoint to return some sort of identifier uuid
    let identifier = match authorized {
      true => Some(resp.text().await?),
      false => None,
    };

    Ok(identifier.map(|s| AuthenticatedBearerIdentifier(s)))
  }

  pub async fn complete(
    self, config: &Config, identifier: AuthenticatedBearerIdentifier,
  ) -> Result<(), ApiError> {
    async fn internal(
      config: &Config, authorization: reqwest::header::HeaderValue,
      identifier: AuthenticatedBearerIdentifier,
    ) -> Result<reqwest::Response, ApiError> {
      let url = config.completion_endpoint();

      let mut headers = reqwest::header::HeaderMap::new();
      headers.append("Authorization", authorization);

      let body = identifier.0;
      let client = reqwest::Client::new();
      let resp = client
        .post(url)
        .body(body)
        .headers(headers)
        .header("Content-Type", "application/json")
        .send()
        .await?;

      Ok(resp)
    }

    // catch any kind of error that may happen:
    match internal(config, self.authorization, identifier.clone()).await {
      Err(e) => {
        println!("{identifier:?} error: {e:?}");
      }
      Ok(_resp) => {}
    };

    Ok(())
  }
}
