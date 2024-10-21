use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => { 
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }        
            EzyTutorError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            EzyTutorError::InvalidInput(msg) => {
                println!("Invalide parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::Error> for EzyTutorError {
    fn from(value: actix_web::Error) -> Self {
        EzyTutorError::ActixError(value.to_string())
    }
}

impl From<SQLxError> for EzyTutorError {
    fn from(value: SQLxError) -> Self {
        EzyTutorError::DBError(value.to_string())
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_msg) | EzyTutorError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_msg) => StatusCode::NOT_FOUND,
            EzyTutorError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

