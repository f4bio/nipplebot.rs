use serde::Deserialize;
use std::fmt;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct ApiError {
  pub status_code: u16,
  pub message: String,
}

impl ApiError {
  pub fn new(status_code: u16, message: String) -> ApiError {
    error!("Api Error: {}/{}", status_code, message);

    ApiError {
      status_code,
      message,
    }
  }
}

impl fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}
//
// impl From<DieselError> for ApiError {
//   fn from(error: DieselError) -> ApiError {
//     match error {
//       DieselError::DatabaseError(_kind, err) => ApiError::new(409, err.message().to_string()),
//       DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
//       err => ApiError::new(500, format!("Diesel error: {}", err)),
//     }
//   }
// }
