use async_graphql::{EmptySubscription, MergedObject, Schema};

use crate::{
    app::core::context::Context,
    model::profile::profile_resolver::{ProfileMutation, ProfileQuery},
};

#[derive(MergedObject, Default)]
pub struct Query(ProfileQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ProfileMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn build_schema_with_context(ctx: Context) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(ctx.profile_service)
        .data(ctx.neodb)
        .enable_subscription_in_federation()
        .finish()
}
