use async_graphql::{EmptySubscription, MergedObject, Schema};
use neo4rs::Graph;
use std::sync::Arc;

use crate::model::profile::profile_resolver::{ProfileMutation, ProfileQuery};

#[derive(MergedObject, Default)]
pub struct Query(ProfileQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ProfileMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema_with_context(neo: Graph) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::new(neo))
        .enable_subscription_in_federation()
        .finish()
}
