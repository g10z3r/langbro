use std::sync::Arc;

use juniper::{EmptySubscription, FieldResult, RootNode};
use mongodb::{
    bson,
    bson::{doc, Document},
    options::ClientOptions,
    results::{DeleteResult, InsertOneResult},
    sync::Client,
    sync::Collection,
};

use crate::app::profile::{NewProfile, Profile};

#[derive(Clone)]
pub struct Context {
    pub mongodb: Arc<mongodb::sync::Client>,
}

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
        // let doc = bson::to_document(&input).unwrap();

        // Ok(context
        //     .mongodb
        //     .database(dotenv!("MONGO_DATABASE_NAME"))
        //     .collection("profiles")
        //     .insert_one(doc, None)
        //     .unwrap())
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
