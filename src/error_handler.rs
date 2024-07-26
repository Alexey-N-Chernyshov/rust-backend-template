use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomError {
    pub error_status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(error_status_code: u16, error_message: impl Into<String>) -> CustomError {
        CustomError {
            error_status_code,
            error_message: error_message.into(),
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code = StatusCode::from_u16(self.error_status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error: ".to_string() + &self.error_message,
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.error_message.as_str())
    }
}

/// Web socket connection error.
impl From<actix_web::Error> for CustomError {
    fn from(error: actix_web::Error) -> CustomError {
        CustomError::new(500, format!("actix_web Error: {error:?}"))
    }
}

/// Custom path error handler that wraps PathError into CustomError in order to get uniform JSON
/// error response.
pub fn path_error_handler(
    err: actix_web::error::PathError,
    _req: &HttpRequest,
) -> actix_web::error::Error {
    CustomError::new(404, err.to_string()).into()
}
