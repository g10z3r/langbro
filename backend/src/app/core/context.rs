use anyhow::Result;
use neo4rs::Graph;
use std::sync::Arc;

use crate::{
    app::db::neo4j,
    model::profile::profile_repository::{ProfileRepository, ProfileRepositoryT},
};

pub struct Context {
    pub neodb: Arc<Graph>,
    pub profile_service: Arc<dyn ProfileRepositoryT>,
}

impl Context {
    pub async fn init() -> Result<Self> {
        let neodb = Arc::new(neo4j::connect().await?);

        Ok(Self {
            profile_service: Arc::new(ProfileRepository::new(&neodb)),
            neodb,
        })
    }
}
