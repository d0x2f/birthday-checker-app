use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;
use firestore::errors::FirestoreError;
use log::error;
use serde_json::json;

#[derive(Debug, Display)]
#[allow(dead_code)]
pub enum AppError {
  #[display(fmt = "not found")]
  NotFound, // 404

  #[display(fmt = "{}", _0)]
  BadRequest(String), // 400

  // Don't expose unknown error text
  #[display(fmt = "Something went wrong!")]
  Other(String), // 500
}

impl ResponseError for AppError {
  fn error_response(&self) -> HttpResponse {
    // Log unknown error
    if let AppError::Other(unknown_error) = self {
      error!("{}", unknown_error)
    }
    HttpResponse::build(self.status_code()).json(json!({ "error": self.to_string() }))
  }

  fn status_code(&self) -> StatusCode {
    match *self {
      AppError::NotFound => StatusCode::NOT_FOUND,
      AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
      AppError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl From<FirestoreError> for AppError {
  fn from(error: FirestoreError) -> Self {
    AppError::Other(error.to_string())
  }
}
