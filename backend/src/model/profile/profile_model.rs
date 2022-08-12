use anyhow::Result;
use argon2::Config;
use async_graphql::{Enum, Object};
use chrono::Utc;
use rand::Rng;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::{
    app::api::security::auth::AuthGuard,
    model::language::language_model::{Language, StudLang},
};

use super::profile_mutation::ProfileRegistrationInput;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Enum, Display, EnumString)]
pub enum Permission {
    #[strum(serialize = "Guest")]
    Guest,

    #[strum(serialize = "User")]
    User,

    #[strum(serialize = "Developer")]
    Developer,

    #[strum(serialize = "Admin")]
    Admin,
}

impl Default for Permission {
    fn default() -> Self {
        Self::Guest
    }
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub(super) id: Uuid,
    pub(super) email: String,
    pub(super) hash: String,
    pub(super) permission: Permission,
    pub(super) username: String,
    pub(super) first_name: String,
    pub(super) last_name: Option<String>,
    pub(super) sex: u8,
    pub(super) age: u8,
    pub(super) description: Option<String>,
    pub(super) native_languages: Vec<Language>,
    pub(super) studied_languages: Vec<StudLang>,
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
}

impl Profile {
    pub fn new(profile_input: ProfileRegistrationInput) -> Result<Self> {
        let profile = Self {
            id: Uuid::new_v4(),
            email: profile_input.email,
            hash: profile_input.password,
            permission: Permission::User,
            username: profile_input.username,
            first_name: profile_input.first_name,
            last_name: profile_input.last_name,
            sex: profile_input.sex,
            age: profile_input.age,
            description: profile_input.description,
            native_languages: Language::from_string_vec(profile_input.native_languages)?,
            studied_languages: profile_input
                .studied_languages
                .into_iter()
                .map(|value| value.into())
                .collect(),
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        };

        Ok(profile.password_hashing()?)
    }

    fn password_hashing(mut self) -> Result<Self> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.hash = argon2::hash_encoded(self.hash.as_bytes(), &salt, &config)?;
        Ok(self)
    }
}

#[Object]
impl<'a> Profile {
    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn id(&'a self) -> String {
        self.id.to_string()
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn email(&'a self) -> &str {
        &self.email
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))")]
    async fn permission(&'a self) -> Permission {
        self.permission
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn username(&'a self) -> &str {
        &self.username
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn first_name(&'a self) -> &str {
        &self.first_name
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn last_name(&'a self) -> &Option<String> {
        &self.last_name
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn sex(&'a self) -> u8 {
        self.sex
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn age(&'a self) -> u8 {
        self.age
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn description(&'a self) -> &Option<String> {
        &self.description
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn native_languages(&'a self) -> &Vec<Language> {
        &self.native_languages
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
    .or(AuthGuard::new(Permission::Developer))
    .or(AuthGuard::new(Permission::User))")]
    async fn studied_languages(&'a self) -> &Vec<StudLang> {
        &self.studied_languages
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn created_at(&'a self) -> i64 {
        self.created_at
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn updated_at(&'a self) -> i64 {
        self.updated_at
    }
}
