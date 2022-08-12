use anyhow::Result;
use neo4rs::{Node, Relation};
use std::str::FromStr;
use uuid::Uuid;

use crate::app::core::error::CustomError;
use crate::model::language::language_model::{CefrKind, Language, StudLang};

use super::profile_model::{Permission, Profile};

pub(super) const NATIVE_SPEAKER: &str = "NATIVE_SPEAKER";
pub(super) const STUDIED: &str = "STUDIED";

impl<'a> Profile {
    /// parse query result
    pub(super) fn pqr(
        pnode: Node,
        rels: Vec<(Relation, Node)>,
    ) -> Result<Profile, CustomError<'a>> {
        let permission = match pnode
            .labels()
            .into_iter()
            .find(|label| Permission::from_str(label).is_ok())
            .map(|label| Permission::from_str(&label).unwrap())
        {
            Some(data) => data,
            None => return Err(crate::internal!("Permission is not found")),
        };

        let mut studied_languages: Vec<StudLang> = Vec::new();
        let mut native_languages: Vec<Language> = Vec::new();

        for (r, n) in rels.iter() {
            match r.typ().as_str() {
                NATIVE_SPEAKER => native_languages
                    .push(Language::try_from(n.get::<String>("name").unwrap().as_str()).unwrap()),

                STUDIED => studied_languages.push(StudLang::new(
                    CefrKind::try_from(r.get::<String>("cefr").unwrap().as_str()).unwrap(),
                    Language::try_from(n.get::<String>("name").unwrap().as_str()).unwrap(),
                )),

                _ => continue,
            }
        }

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
            native_languages,
            studied_languages,
            created_at: pnode.get::<i64>("created_at").unwrap(),
            updated_at: pnode.get::<i64>("updated_at").unwrap(),
        })
    }
}
