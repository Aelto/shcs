use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ApiError {
    Storage(storage::StorageError),
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

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Storage(e) => write!(f, "storage error: {e}"),
            Self::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::Storage(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            ApiError::Storage(_) => HttpResponse::InternalServerError().finish(),
            ApiError::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}
