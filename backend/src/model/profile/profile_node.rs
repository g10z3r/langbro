use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

use super::profile_model::Profile;

pub(super) struct ProfileNode {
    props: Props,
    labels: Vec<String>,
    // relations: HashMap<String, String>,
}

impl ProfileNode {
    fn new(node: neo4rs::Node) -> Result<Self> {
        todo!()
    }
}

pub(super) struct Props {
    pub(super) id: Uuid,
    pub(super) email: String,
    pub(super) hash: String,
    pub(super) username: String,
    pub(super) first_name: String,
    pub(super) last_name: Option<String>,
    pub(super) sex: u8,
    pub(super) age: u8,
    pub(super) description: Option<String>,
    pub(super) created_at: i64,
    pub(super) updated_at: i64,
}
