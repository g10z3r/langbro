use std::str::FromStr;

use anyhow::Result;
use argon2::Config;
use async_graphql::{Enum, Object};
use chrono::Utc;
use rand::Rng;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

use crate::app::api::security::auth::AuthGuard;

use super::profile_mutation::ProfileRegistrationInput;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Enum, Display, EnumString)]
pub enum Permissions {
    #[strum(serialize = "Guest")]
    Guest,

    #[strum(serialize = "User")]
    User,

    #[strum(serialize = "Developer")]
    Developer,

    #[strum(serialize = "Admin")]
    Admin,
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub(super) id: Uuid,
    pub(super) email: String,
    pub(super) hash: String,
    pub(super) permissions: Permissions,
    pub(super) username: String,
    pub(super) first_name: String,
    pub(super) last_name: Option<String>,
    pub(super) sex: u8,
    pub(super) age: u8,
    pub(super) description: Option<String>,
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
}

impl Profile {
    pub fn new(profile_input: ProfileRegistrationInput) -> Result<Self> {
        let profile = Self {
            id: Uuid::new_v4(),
            email: profile_input.email,
            hash: profile_input.password,
            permissions: Permissions::User,
            username: profile_input.username,
            first_name: profile_input.first_name,
            last_name: profile_input.last_name,
            sex: profile_input.sex,
            age: profile_input.age,
            description: profile_input.description,
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

impl Profile {
    pub(super) fn from_node(node: neo4rs::Node) -> Result<Self> {
        Ok(Self {
            id: Uuid::parse_str(&node.get::<String>("id").expect("Faild to get node id"))?,
            email: node.get::<String>("email").expect("Faild to get node id"),
            hash: node.get::<String>("hash").expect("Faild to get node id"),
            permissions: Permissions::from_str(&node.labels()[1])?,
            username: node
                .get::<String>("username")
                .expect("Faild to get node id"),
            first_name: node
                .get::<String>("first_name")
                .expect("Faild to get node id"),
            last_name: node.get::<String>("last_name"),
            sex: node.get::<i64>("sex").expect("Faild to get node id") as u8,
            age: node.get::<i64>("age").expect("Faild to get node id") as u8,
            description: node.get::<String>("description"),
            created_at: node.get::<i64>("created_at").expect("Faild to get node id"),
            updated_at: node.get::<i64>("updated_at").expect("Faild to get node id"),
        })
    }
}

#[Object]
impl<'a> Profile {
    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn id(&'a self) -> String {
        self.id.to_string()
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn email(&'a self) -> &str {
        &self.email
    }

    #[graphql(guard = "AuthGuard::new(Permissions::Admin)")]
    async fn permissions(&'a self) -> Permissions {
        self.permissions
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn username(&'a self) -> &str {
        &self.username
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn first_name(&'a self) -> &str {
        &self.first_name
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn last_name(&'a self) -> &Option<String> {
        &self.last_name
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn sex(&'a self) -> u8 {
        self.sex
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn age(&'a self) -> u8 {
        self.age
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn description(&'a self) -> &Option<String> {
        &self.description
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn created_at(&'a self) -> i64 {
        self.created_at
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn updated_at(&'a self) -> i64 {
        self.updated_at
    }
}
