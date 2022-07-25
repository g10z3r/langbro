use std::{rc::Rc, sync::Arc};

use juniper::{graphql_value, EmptySubscription, FieldError, FieldResult, RootNode};
use mongodb::bson;

use crate::app::account::{Account, NewAccount};

pub struct Context {
    pub mongodb: Arc<mongodb::sync::Database>,
}

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    pub async fn accounts() -> Vec<Account> {
        todo!()
    }
}

pub struct Mutation;

#[juniper::graphql_object(context = Context)]
impl Mutation {
    pub async fn create_account(input: NewAccount, context: &Context) -> FieldResult<Account> {
        let mut account = Account::new(Rc::new(input))?;
        let doc = bson::to_document(account.password_hashing()?)?;
        let collection = context.mongodb.collection("accounts");

        match collection.insert_one(doc, None) {
            Ok(_) => Ok(account),
            Err(err) => {
                let details = err.to_string();
                Err(FieldError::new(
                    "Failed to create new account",
                    graphql_value!({ "details": details }),
                ))
            }
        }
    }
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::<Context>::new())
}
