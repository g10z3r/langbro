use async_graphql::{Context, Object, Result as GraphQLResult};
use std::sync::Arc;
use validator::Validate;

use crate::app::api::security::auth::{self, AccessClaims, Token};
use crate::app::core::error::CustomError;
use crate::model::profile::profile_resolver::auth::AuthGuard;

use super::profile_repository::ProfileRepositoryT;
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
        profile_input.validate()?;

        let profile = Profile::new(profile_input)?;
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;

        profile_service.create(profile).await?;

        Ok("OK")
    }

    async fn login(
        &'a self,
        ctx: &'a Context<'_>,
        login_input: ProfileLoginInput,
    ) -> GraphQLResult<ProfileLoginOutput> {
        login_input.validate()?;

        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let profile = profile_service
            .get_by_username(login_input.username)
            .await?;

        let access_token = Token::encode(auth::AccessClaims::new(
            profile.id.to_string(),
            profile.permissions,
            chrono::Duration::minutes(15),
        ))?;
        let refresh_token = Token::encode(auth::RefreshClaims::new(
            profile.id.to_string(),
            chrono::Duration::days(7),
        ))?;

        Ok(ProfileLoginOutput::create(access_token, refresh_token))
    }

    #[graphql(guard = "AuthGuard::new(Permissions::Admin)
        .or(AuthGuard::new(Permissions::Developer))
        .or(AuthGuard::new(Permissions::User))")]
    async fn subscribe(&'a self, ctx: &'a Context<'_>, to_id: String) -> GraphQLResult<&str> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let access_claims = ctx
            .data_opt::<Result<Option<AccessClaims>, CustomError>>()
            .unwrap()
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();

        profile_service
            .subscribe(to_id, access_claims.sub().to_string())
            .await?;

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
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;

        Ok(profile_service.get_by_id(id).await?)
    }
}
