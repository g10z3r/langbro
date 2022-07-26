use std::rc::Rc;
use std::str::FromStr;

use argon2::Config;
use juniper::GraphQLInputObject;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime as MongoDateTime;
use rand::Rng;
use validator::Validate;

use crate::core::error::AppError;
use crate::core::language::{CefrInput, Language, CEFR};
use crate::core::validation::validate_query;
use crate::err_internal;

#[derive(Debug, Clone, Serialize)]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub sex: u8,
    pub hash: String,
    pub description: Option<String>,
    pub native_language: Vec<Language>,
    pub study_language: Vec<CEFR>,
    pub friends: Option<Vec<Account>>,
    pub created_at: MongoDateTime,
    pub updated_at: MongoDateTime,
}

#[derive(Clone, Validate, Serialize, Deserialize, GraphQLInputObject)]
pub struct NewAccount {
    #[validate(
        length(min = 4, max = 10, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub sex: i32,
    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    pub password: String,
    pub description: Option<String>,
    pub native_language: Vec<String>,
    pub study_language: Vec<CefrInput>,
}

impl Account {
    pub fn new(new_account: Rc<NewAccount>) -> Result<Account, AppError> {
        Ok(Self {
            id: ObjectId::new(),
            username: new_account.username.clone(),
            first_name: new_account.first_name.clone(),
            last_name: new_account.last_name.clone(),
            hash: new_account.password.to_string(),
            sex: new_account.sex as u8,
            description: new_account.description.clone(),
            native_language: Self::parse_native_language(&new_account.native_language)?,
            study_language: Self::parse_study_language(&new_account.study_language),
            friends: None,
            created_at: MongoDateTime::now(),
            updated_at: MongoDateTime::now(),
        })
    }

    fn parse_native_language(list: &Vec<String>) -> Result<Vec<Language>, AppError> {
        let mut new_list: Vec<Language> = Vec::new();

        for language in list.iter() {
            new_list.push(Language::from_str(language)?);
        }

        Ok(new_list)
    }

    fn parse_study_language(list: &Vec<CefrInput>) -> Vec<CEFR> {
        let mut new_list: Vec<CEFR> = Vec::new();

        for item in list.into_iter() {
            new_list.push(item.clone().into());
        }

        new_list
    }
}

impl Account {
    pub fn password_verify(&self, password: &[u8]) -> Result<bool, AppError> {
        argon2::verify_encoded(&self.hash, password)
            .map_err(|_| err_internal!("Failed to verify password".to_string()))
    }

    pub fn password_hashing(&mut self) -> Result<&Account, AppError> {
        self.hash = Self::password_hashing_apart(&self.hash)?;
        Ok(self)
    }

    fn password_hashing_apart(password: &str) -> Result<String, AppError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config)
            .map_err(|_| err_internal!("Failed create password hash".to_string()))?;

        Ok(hash)
    }
}

#[juniper::graphql_object]
impl Account {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn sex(&self) -> i32 {
        self.sex as i32
    }

    pub fn native_language(&self) -> Vec<Language> {
        self.native_language.clone()
    }

    pub fn study_language(&self) -> Vec<CEFR> {
        self.study_language.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn friends(&self) -> Option<Vec<Account>> {
        self.friends.clone()
    }

    pub fn created_at(&self) -> String {
        self.created_at.to_string()
    }

    pub fn updated_at(&self) -> String {
        self.updated_at.to_string()
    }
}
