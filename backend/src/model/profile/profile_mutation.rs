use async_graphql::{InputObject, Object};
use validator::Validate;

use crate::app::utils::{regex::RE_NAME, validation::validate_query};

#[derive(Validate, Serialize, Deserialize, InputObject)]
pub struct ProfileRegistrationInput {
    #[validate(email)]
    pub(super) email: String,

    #[validate(
        length(min = 4, max = 10, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub(super) username: String,

    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub(super) password: String,

    #[validate(
        length(min = 2, max = 10, message = "Lenght is invalid"),
        regex = "RE_NAME"
    )]
    pub(super) first_name: String,

    #[validate(
        length(min = 2, max = 10, message = "Lenght is invalid"),
        regex = "RE_NAME"
    )]
    pub(super) last_name: Option<String>,

    #[validate(range(min = 0, max = 1))]
    pub(super) sex: u8,

    #[validate(range(min = 18, max = 99))]
    pub(super) age: u8,

    pub(super) description: Option<String>,

    #[validate(length(min = 1, max = 3, message = "Lenght is invalid"))]
    pub(super) native_languages: Vec<String>,
}

#[derive(Validate, Serialize, Deserialize, InputObject)]
pub struct ProfileLoginInput {
    #[validate(
        length(min = 4, max = 10, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub(super) username: String,
    
    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub(super) password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProfileLoginOutput {
    access_token: String,
    refresh_token: String,
}

impl<'a> ProfileLoginOutput {
    pub(super) fn create(access_token: String, refresh_token: String) -> Self {
        ProfileLoginOutput {
            access_token,
            refresh_token,
        }
    }
}

#[Object]
impl<'a> ProfileLoginOutput {
    async fn access_token(&'a self) -> &str {
        &self.access_token
    }

    async fn refresh_token(&'a self) -> &str {
        &self.refresh_token
    }
}
