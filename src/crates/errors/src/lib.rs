use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use derive_more::derive::{Display, Error};
use serde_json::json;

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display("InvalidStateError: {message}")]
    InvalidStateError { message: &'static str },
    #[display("NotFoundError: {message}")]
    NotFoundError { message: &'static str },
    #[display("UnauthorizedError: Invalid login or password")]
    UnauthorizedError,
    #[display("ForbiddenError: Forbidden!")]
    ForbiddenError,
    #[display("DatabaseError")]
    DatabaseError,
    #[display("PasswordError")]
    PasswordError,
    #[display("InternalServerError")]
    InternalServerError,
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::InvalidStateError { .. } => StatusCode::BAD_REQUEST,
            Error::NotFoundError { .. } => StatusCode::NOT_FOUND,
            Error::ForbiddenError => StatusCode::FORBIDDEN,
            Error::UnauthorizedError => StatusCode::UNAUTHORIZED,
            Error::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::PasswordError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({
            "code": self.status_code().to_string(),
            "message": self.to_string()
        }))
    }
}
