use juniper::{EmptySubscription, FieldResult, RootNode};

use crate::app::profile::{NewProfile, Profile};

#[derive(Clone)]
pub struct Context {}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    pub async fn profiles() -> Vec<Profile> {
        vec![
            Profile {
                id: "3r54fwrty".to_string(),
                username: "test1".to_string(),
            },
            Profile {
                id: "3r54fwrty".to_string(),
                username: "test2".to_string(),
            },
        ]
    }
}

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    pub async fn new_profile(input: NewProfile, context: &Context) -> FieldResult<Profile> {
        todo!()
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
