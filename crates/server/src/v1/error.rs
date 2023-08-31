use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ApiError {
  Storage(storage::StorageError),
  InternalServerError,
  Unauthorized,
}

impl From<storage::StorageError> for ApiError {
  fn from(value: storage::StorageError) -> Self {
    Self::Storage(value)
  }
}

impl From<std::io::Error> for ApiError {
  fn from(value: std::io::Error) -> Self {
    Self::Storage(storage::StorageError::Io(value))
  }
}

impl From<actix_web::error::BlockingError> for ApiError {
  fn from(value: actix_web::error::BlockingError) -> Self {
    Self::InternalServerError
  }
}

impl std::fmt::Display for ApiError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ApiError::Storage(e) => write!(f, "storage error: {e}"),
      Self::Unauthorized => write!(f, "unauthorized"),
      Self::InternalServerError => write!(f, "internal server error"),
    }
  }
}

impl actix_web::ResponseError for ApiError {
  fn status_code(&self) -> actix_web::http::StatusCode {
    match self {
      ApiError::Storage(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
      ApiError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
    match self {
      ApiError::Storage(_) => HttpResponse::InternalServerError().finish(),
      ApiError::Unauthorized => HttpResponse::Unauthorized().finish(),
      ApiError::InternalServerError => HttpResponse::InternalServerError().finish(),
    }
  }
}
