use async_graphql::{Context, Error as GraphQLError, Object, Result as GraphQLResult};
use chrono::Utc;
use neo4rs::{Graph, Node};

use std::sync::Arc;

use crate::app::core::error::CustomError;
use crate::model::profile::profile_resolver::auth::AuthGuard;
use crate::{
    app::api::security::auth::{self, AccessClaims, Token},
    neo4j_result,
};

use super::profile_service::ProfileServiceT;
use super::{
    profile_model::{Permissions, Profile},
    profile_mutation::{ProfileLoginInput, ProfileLoginOutput, ProfileRegistrationInput},
};

#[derive(Default)]
pub struct ProfileMutation;

#[Object]
impl<'a> ProfileMutation {
    async fn registration(
        &'a self,
        ctx: &'a Context<'_>,
        profile_input: ProfileRegistrationInput,
    ) -> GraphQLResult<&str> {
        let profile = Profile::new(profile_input)?;
        let profile_service = ctx.data::<Arc<dyn ProfileServiceT>>()?;

        profile_service.create(profile).await?;

        Ok("OK")
    }

    async fn login(
        &'a self,
        ctx: &'a Context<'_>,
        login_input: ProfileLoginInput,
    ) -> GraphQLResult<ProfileLoginOutput> {
        let neo = ctx.data::<Arc<Graph>>()?;
        let query = neo4rs::query("MATCH (p:Person {username: $username}) RETURN p")
            .param("username", login_input.username.clone());
        let mut result = neo.execute(query).await.expect("Faild to get record");

        match neo4j_result!(result.next().await)? {
            Some(row) => {
                let node = row.get::<Node>("p").unwrap();
                let access_token = Token::encode(auth::AccessClaims::new(
                    node.get::<String>("id").unwrap(),
                    Permissions::User,
                    chrono::Duration::minutes(15),
                ))?;
                let refresh_token = Token::encode(auth::RefreshClaims::new(
                    login_input.username,
                    chrono::Duration::days(7),
                ))?;

                Ok(ProfileLoginOutput::create(access_token, refresh_token))
            }

            None => Err(GraphQLError::new("User was not found")),
        }
    }

    #[graphql(guard = "AuthGuard::new(Permissions::User)")]
    async fn subscribe(&'a self, ctx: &'a Context<'_>, to_id: String) -> GraphQLResult<&str> {
        let access_claims = ctx
            .data_opt::<Result<Option<AccessClaims>, CustomError>>()
            .unwrap()
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();

        let neo = ctx.data::<Arc<Graph>>()?;
        let query = neo4rs::query(
            "
                MATCH (e:Person) WHERE e.id = $form
                MATCH (d:Person) WHERE d.id = $to

                CREATE (e)-[:SUBSCRIBE {timestamp: $timestamp}]->(d)
            ",
        )
        .param("form", access_claims.sub())
        .param("to", to_id)
        .param("timestamp", Utc::now().timestamp());

        neo4j_result!(neo.run(query).await)?;

        Ok("OK")
    }
}

#[derive(Default)]
pub struct ProfileQuery;

#[Object]
impl<'a> ProfileQuery {
    async fn get_profile_by_id(
        &'a self,
        ctx: &'a Context<'_>,
        id: String,
    ) -> GraphQLResult<Profile> {
        let neo = ctx.data::<Arc<Graph>>()?;
        let query = neo4rs::query("MATCH (p:Person {id: $id}) RETURN p").param("id", id);

        let mut result = neo4j_result!(neo.execute(query).await)?;
        let row = result.next().await.unwrap().unwrap();
        let node = row.get::<Node>("p").expect("Faild to get node");

        Ok(Profile::from_node(node)?)
    }
}
