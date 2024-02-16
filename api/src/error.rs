use std::{collections::HashMap, fmt::Display};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug)]
pub enum DateTimeError {
    InvalidDateTime(String),
    InvalidZone(String),
}

impl Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DateTimeError::InvalidDateTime(s) => s,
            DateTimeError::InvalidZone(s) => s,
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Error {
    DateTime(DateTimeError),
    Function(String),
}

impl From<DateTimeError> for Error {
    fn from(value: DateTimeError) -> Self {
        Self::DateTime(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::DateTime(e) => match e {
                DateTimeError::InvalidDateTime(s) => s,
                DateTimeError::InvalidZone(s) => s,
            },
            Error::Function(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::DateTime(_) => StatusCode::BAD_REQUEST,
            Error::Function(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut result = HashMap::new();
        result.insert("error", format!("{}", self));
        HttpResponse::build(self.status_code()).json(result)
    }
}
