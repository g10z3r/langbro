use actix_web::http::header::ToStrError;
use async_graphql::{Error as GraphQLError, ErrorExtensions as GraphQLErrorExtensions};
use strum::ParseError;
use thiserror::Error;
use validator::ValidationErrors;

#[macro_export]
macro_rules! neo4j_result {
    ( $x:expr ) => {{
        use crate::app::core::error::handle_neo4j_result;
        handle_neo4j_result($x)
    }};
}

#[derive(Debug, Clone, Error, Serialize)]
pub enum CustomErrorKind {
    #[error("Could not find resource")]
    NotFound,

    #[error("Resource access denied")]
    Forbidden,

    #[error("The received data is not valid")]
    Unprocessable,

    #[error("An internal error has occurred")]
    Internal,

    #[error("The passed token is expired")]
    TokenExpired,

    #[error("Request header missing token")]
    TokenMissing,

    #[error("The passed token is not valid")]
    TokenInvalid,

    #[error("An unknown error has occurred")]
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct CustomError {
    message: String,
    details: Option<String>,
    kind: CustomErrorKind,
}

impl CustomError {
    pub fn new() -> CustomErrorBuilder {
        CustomErrorBuilder::new()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn details(&self) -> Option<String> {
        self.details.clone()
    }

    pub fn kind(&self) -> CustomErrorKind {
        self.kind.clone()
    }

    pub fn serde_kind(&self) -> &str {
        match self.kind {
            CustomErrorKind::NotFound => "NOT_FOUND",
            CustomErrorKind::Forbidden => "FORBIDDEN",
            CustomErrorKind::Unprocessable => "UNPROCESSABLE",
            CustomErrorKind::Internal => "INTERNAL",
            CustomErrorKind::TokenExpired => "TOKEN_EXPIRED",
            CustomErrorKind::TokenMissing => "TOKEN_MISSING",
            CustomErrorKind::TokenInvalid => "TOKEN_INVALID",
            CustomErrorKind::Unknown => "UNKNOWN",
        }
    }
}

impl From<ToStrError> for CustomError {
    fn from(source: ToStrError) -> Self {
        use self::CustomErrorKind::Internal;

        CustomError::new()
            .kind(Internal)
            .details(&source.to_string())
            .build()
    }
}

impl From<ParseError> for CustomError {
    fn from(source: ParseError) -> Self {
        use self::CustomErrorKind::Unprocessable;

        CustomError::new()
            .kind(Unprocessable)
            .details(&source.to_string())
            .build()
    }
}

impl From<ValidationErrors> for CustomError {
    fn from(err: ValidationErrors) -> Self {
        use self::CustomErrorKind::Unprocessable;

        CustomError::new()
            .kind(Unprocessable)
            .details(&(err.to_string()))
            .build()
    }
}

pub struct CustomErrorBuilder {
    details: Option<String>,
    kind: CustomErrorKind,
}

impl Default for CustomErrorBuilder {
    fn default() -> Self {
        Self {
            details: None,
            kind: CustomErrorKind::Internal,
        }
    }
}

impl CustomErrorBuilder {
    fn new() -> Self {
        Self::default()
    }

    pub fn kind(mut self, kind: CustomErrorKind) -> Self {
        self.kind = kind;

        self
    }

    pub fn details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());

        self
    }

    pub fn some_details(mut self, details: Option<String>) -> Self {
        match details {
            Some(data) => {
                self.details = Some(data);

                self
            }

            None => self,
        }
    }

    pub fn build(self) -> CustomError {
        CustomError {
            message: self.kind.to_string(),
            details: self.details,
            kind: self.kind,
        }
    }
}

impl From<CustomError> for GraphQLError {
    fn from(err: CustomError) -> Self {
        GraphQLError::new(&err.message)
            .extend_with(|_, e| e.set("kind", err.serde_kind()))
            .extend_with(|_, e| e.set("details", err.details.unwrap_or("".to_string())))
    }
}

impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use self::CustomErrorKind::{Internal, TokenExpired, TokenInvalid};

        let custom_error = CustomError::new();

        match *err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                custom_error.kind(TokenInvalid).build()
            }
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                custom_error.kind(TokenExpired).build()
            }
            _ => custom_error
                .kind(Internal)
                .details("Failed to decode token")
                .build(),
        }
    }
}

impl From<neo4rs::Error> for CustomError {
    fn from(err: neo4rs::Error) -> Self {
        use self::CustomErrorKind::Internal;

        let custom_error = CustomError::new().kind(Internal);

        match err {
            neo4rs::Error::IOError { detail } => custom_error.details(&detail).build(),
            neo4rs::Error::ConnectionError => custom_error.details("Connection error").build(),
            neo4rs::Error::StringTooLong => custom_error.details("String to long").build(),
            neo4rs::Error::MapTooBig => custom_error.details("Map too big").build(),
            neo4rs::Error::BytesTooBig => custom_error.details("Bytes too big").build(),
            neo4rs::Error::ListTooLong => custom_error.details("List too long").build(),
            neo4rs::Error::InvalidConfig => custom_error.details("Invalid config").build(),
            neo4rs::Error::UnsupportedVersion(err) => custom_error.details(&err).build(),
            neo4rs::Error::UnexpectedMessage(err) => custom_error.details(&err).build(),
            neo4rs::Error::UnknownType(err) => custom_error.details(&err).build(),
            neo4rs::Error::UnknownMessage(err) => custom_error.details(&err).build(),
            neo4rs::Error::ConverstionError => custom_error.details("Converstion error").build(),
            neo4rs::Error::AuthenticationError(err) => custom_error.details(&err).build(),
            neo4rs::Error::InvalidTypeMarker(err) => custom_error.details(&err).build(),
            neo4rs::Error::DeserializationError(err) => custom_error.details(&err).build(),
        }
    }
}

pub fn handle_neo4j_result<T>(neo4j_result: Result<T, neo4rs::Error>) -> Result<T, CustomError> {
    match neo4j_result {
        Ok(data) => Ok(data),
        Err(err) => Err(err.into()),
    }
}
