use juniper::{GraphQLObject, DefaultScalarValue};
use mongodb::bson::DateTime as MongoDateTime;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;
use juniper::GraphQLInputObject;
use validator::Validate;

use crate::core::error::AppError;

#[derive(Debug, Serialize)]
pub struct Profile {
    #[serde(rename = "_id")]
    pub id: String,
    pub username: String,
//     pub hash: String,

//     pub created_at: MongoDateTime,
//     pub updated_at: MongoDateTime,
}

#[derive(Validate, Deserialize, GraphQLInputObject)]
pub struct NewProfile {
    pub username: String,
    pub password: String,
}

// impl Profile {
//     pub fn password_verify(&self, password: &[u8]) -> Result<bool, AppError> {
//         argon2::verify_encoded(&self.hash, password).map_err(|err| {
//             todo!()
//             // HubError::new_internal("Failed to verify password", Some(Vec::new()))
//             //     .add(format!("{}", err))
//         })
//     }
// }

#[juniper::graphql_object]
impl Profile {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    // pub fn created_at(&self) -> i64 {
        
    //     self.created_at.timestamp_millis()
    // }
}