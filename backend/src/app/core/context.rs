use anyhow::Result;
use neo4rs::Graph;
use std::sync::Arc;

use crate::{
    app::db::neo4j,
    model::profile::profile_service::{ProfileService, ProfileServiceT},
};

pub struct Context {
    pub neodb: Arc<Graph>,
    pub profile_service: Arc<dyn ProfileServiceT>,
}

impl Context {
    pub async fn init() -> Result<Self> {
        let neodb = Arc::new(neo4j::connect().await?);

        Ok(Self {
            profile_service: Arc::new(ProfileService::new(&neodb)),
            neodb,            
        })
    }
}
