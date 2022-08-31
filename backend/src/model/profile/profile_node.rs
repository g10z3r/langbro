use anyhow::Result;
use neo4rs::Node;
use std::str::FromStr;
use uuid::Uuid;

use crate::app::core::error::CustomError;

use super::profile_model::{Permission, Profile};

pub(super) const NATIVE_SPEAKER: &str = "NATIVE_SPEAKER";
pub(super) const STUDIED: &str = "STUDIED";

impl<'a> Profile {
    pub(super) fn parse_query_resp(pnode: Node) -> Result<Profile, CustomError<'a>> {
        let permission = match pnode
            .labels()
            .into_iter()
            .find(|label| Permission::from_str(label).is_ok())
            .map(|label| Permission::from_str(&label).unwrap())
        {
            Some(data) => data,
            None => Permission::User,
        };

        Ok(Profile {
            id: Uuid::parse_str(&pnode.get::<String>("id").unwrap())?,
            email: pnode.get::<String>("email").unwrap(),
            hash: pnode.get::<String>("hash").unwrap(),
            permission,
            username: pnode.get::<String>("username").unwrap(),
            first_name: pnode.get::<String>("first_name").unwrap(),
            last_name: pnode.get::<String>("last_name"),
            sex: pnode.get::<i64>("sex").unwrap() as u8,
            age: pnode.get::<i64>("age").unwrap() as u8,
            description: pnode.get::<String>("description"),
            created_at: pnode.get::<i64>("created_at").unwrap(),
            updated_at: pnode.get::<i64>("updated_at").unwrap(),
        })
    }
}
