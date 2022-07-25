use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    Validation(String),
}

#[macro_export]
macro_rules! err_internal {
    ( $x:expr ) => {
        AppError::Internal($x)
    };
}

#[macro_export]
macro_rules! err_validation {
    ( $x:expr ) => {
        AppError::Validation($x)
    };
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<S: ScalarValue> IntoFieldError<S> for AppError {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            AppError::Internal(text) => FieldError::new(text, graphql_value!(None)),
            AppError::Validation(details) => FieldError::new(
                "Validation failed",
                graphql_value!({ "details": details }),
            ),
        }
    }
}
