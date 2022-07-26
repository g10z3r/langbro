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
use crate::core::regex::RE_NAME;
use crate::core::validation::validate_query;
use crate::err_internal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "_id")]
    id: ObjectId,
    username: Option<String>,
    email: String,
    first_name: String,
    last_name: Option<String>,
    sex: u8,
    age: u8,
    hash: String,
    description: Option<String>,
    native_language: Vec<Language>,
    study_language: Vec<CEFR>,
    friends: Option<Vec<Account>>,
    created_at: MongoDateTime,
    updated_at: MongoDateTime,
}

#[derive(Clone, Validate, Serialize, Deserialize, GraphQLInputObject)]
pub struct NewAccount {
    #[validate(
        length(min = 4, max = 10, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    username: Option<String>,

    #[validate(email)]
    email: String,

    #[validate(
        length(min = 2, max = 10, message = "Lenght is invalid"),
        regex = "RE_NAME"
    )]
    first_name: String,

    #[validate(
        length(min = 2, max = 10, message = "Lenght is invalid"),
        regex = "RE_NAME"
    )]
    last_name: Option<String>,

    #[validate(range(min = 0, max = 1))]
    sex: i32,

    #[validate(range(min = 18, max = 99))]
    age: i32,

    #[validate(
        length(min = 8, max = 20, message = "Lenght is invalid"),
        custom(function = "validate_query", message = "Invalid format")
    )]
    password: String,

    #[validate(length(min = 1, max = 288, message = "Lenght is invalid"))]
    description: Option<String>,

    #[validate(length(min = 1, max = 3, message = "Lenght is invalid"))]
    native_language: Vec<String>,

    #[validate(length(min = 1, max = 4, message = "Lenght is invalid"))]
    study_language: Vec<CefrInput>,
}

impl Account {
    pub fn new(new_account: Rc<NewAccount>) -> Result<Account, AppError> {
        Ok(Self {
            id: ObjectId::new(),
            username: new_account.username.clone(),
            email: new_account.email.clone(),
            first_name: new_account.first_name.clone(),
            last_name: new_account.last_name.clone(),
            hash: new_account.password.to_string(),
            sex: new_account.sex as u8,
            age: new_account.age as u8,
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

    pub fn email(&self) -> String {
        self.email.clone()
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

    pub fn age(&self) -> i32 {
        self.age as i32
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
