use async_graphql::{InputObject, Object};

#[derive(Serialize, Deserialize, InputObject)]
pub struct ProfileRegistrationInput {
    pub(super) email: String,
    pub(super) password: String,
    pub(super) username: String,
    pub(super) first_name: String,
    pub(super) last_name: Option<String>,
    pub(super) sex: u8,
    pub(super) age: u8,
    pub(super) description: Option<String>,
}

#[derive(Serialize, Deserialize, InputObject)]
pub struct ProfileLoginInput {
    pub(super) username: String,
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
