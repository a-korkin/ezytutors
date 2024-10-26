use std::fmt;
use serde::Serialize;
use actix_web::{error, http::StatusCode, HttpResponse};
use sqlx::Error as SQLxError;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    TeraError(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            EzyTutorError::DBError(_msg) |
            EzyTutorError::ActixError(_msg) |
            EzyTutorError::TeraError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            EzyTutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl std::error::Error for EzyTutorError {}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzyTutorError::ActixError(msg) => {
                println!("Actix error occurred: {:?}", msg);
                "Actix error".into()
            }
            EzyTutorError::TeraError(msg) => {
                println!("Tera error occurred: {:?}", msg);
                "Tera error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                "Not found error".into()
            }
        }
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzyTutorError {
    fn from(err: SQLxError) -> Self {
        EzyTutorError::DBError(err.to_string())
    }
}

