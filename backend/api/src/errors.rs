use rocket::{Request, Response as RocketResponse, http::Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use application::errors::ApplicationError;
use shared::response_models::Response;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Invalid input data: {0}")]
    ValidationError(String),  // For input validation errors

    #[error("Internal server error: {0}")]
    InternalError(String),    // For unexpected errors at the API level

    #[error("Not found: {0}")]
    NotFoundError(String),    // For handling 404 Not Found
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::DatabaseError(msg) => ApiError::InternalError(msg),
            ApplicationError::NotFound(msg) => ApiError::NotFoundError(msg), // Map to 404
        }
    }
}

impl<'r> Responder<'r, 'r> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'r> {
        let (status, error_message) = match self {
            ApiError::ValidationError(msg) => (Status::BadRequest, msg),
            ApiError::InternalError(msg) => (Status::InternalServerError, msg),
            ApiError::NotFoundError(msg) => (Status::NotFound, msg),
        };

        let error_response: Json<Response<()>> = Json(Response::error(error_message));
        let response_body = match serde_json::to_string(&error_response.into_inner()) {
            Ok(body) => body,
            Err(_) => return Err(Status::InternalServerError),
        };

        RocketResponse::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(response_body.len(), std::io::Cursor::new(response_body))
            .ok()
    }
}

