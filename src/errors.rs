use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use juniper::graphql_value;
use std::convert::From;
use uuid::parser::ParseError;

#[derive(Debug, Display, Serialize)]
pub enum $name_pascal_case$Error {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl juniper::IntoFieldError for $name_pascal_case$Error {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            $name_pascal_case$Error::Unauthorized => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "NO_ACCESS"
                }),
            ),

            _ => juniper::FieldError::new(
                "Unknown Error",
                graphql_value!({
                    "type": "UNKNOWN_ERROR"
                }),
            ),
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for $name_pascal_case$Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            $name_pascal_case$Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            $name_pascal_case$Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            $name_pascal_case$Error::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for $name_pascal_case$Error {
    fn from(_: ParseError) -> $name_pascal_case$Error {
        $name_pascal_case$Error::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for $name_pascal_case$Error {
    fn from(error: DBError) -> $name_pascal_case$Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return $name_pascal_case$Error::BadRequest(message);
                }
                $name_pascal_case$Error::InternalServerError
            }
            _ => $name_pascal_case$Error::InternalServerError,
        }
    }
}
