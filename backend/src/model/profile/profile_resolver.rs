use async_graphql::{Context, Object, Result as GraphQLResult};
use std::sync::Arc;
use validator::Validate;

use crate::app::api::security::auth::{self, AccessClaims, Token};
use crate::app::core::error::CustomError;
use crate::model::language::language_model::{CefrKind, Language};
use crate::model::profile::profile_resolver::auth::AuthGuard;

use super::profile_mutation::EditProfileInput;
use super::profile_repository::ProfileRepositoryT;
use super::{
    profile_model::{Permission, Profile},
    profile_mutation::{ProfileLoginInput, ProfileLoginOutput, ProfileRegistrationInput},
};

#[derive(Default)]
pub struct ProfileMutation;

#[Object]
impl<'a> ProfileMutation {
    /// Метод для регистрации.
    async fn registration(
        &'a self,
        ctx: &'a Context<'_>,
        profile_input: ProfileRegistrationInput,
    ) -> GraphQLResult<&str> {
        profile_input.validate()?;

        let profile = Arc::new(Profile::new(profile_input)?);
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;

        profile_service.create(profile).await?;

        Ok("OK")
    }

    /// Метод для авторизации.
    /// В ответ клиент должен получить набор токенов.
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
            profile.permission,
            chrono::Duration::minutes(15),
        ))?;
        let refresh_token = Token::encode(auth::RefreshClaims::new(
            profile.id.to_string(),
            chrono::Duration::days(7),
        ))?;

        Ok(ProfileLoginOutput::create(access_token, refresh_token))
    }

    /// Метод для удаления связи :SUBSCRIBE между двумя узлами типа :Profile
    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn unsubscribe(&'a self, ctx: &'a Context<'_>, from_id: String) -> GraphQLResult<&str> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let access_claims = get_access_claims(ctx);

        profile_service
            .unsubscribe(access_claims.sub().to_string(), from_id)
            .await?;

        Ok("OK")
    }

    /// Метод для установки связи :SUBSCRIBE между двумя узлами типа :Profile
    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn subscribe(&'a self, ctx: &'a Context<'_>, to_id: String) -> GraphQLResult<&str> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let access_claims = get_access_claims(ctx);

        if access_claims.sub().to_string() != to_id {
            profile_service
                .subscribe(to_id, access_claims.sub().to_string())
                .await?;

            Ok("OK")
        } else {
            Err(crate::unprocessable!("id", Some("You can't follow yourself".to_string())).into())
        }
    }

    /// Метод для удаления связи между узлом :Profile и :Language.
    ///
    /// В качестве параметра должно передаваться тип связи между
    /// двумя указанными узлами и параметр `name` для узла :Language.
    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn remove_lang_rel(
        &'a self,
        ctx: &'a Context<'_>,
        rel_type: String,
        lang: Language,
    ) -> GraphQLResult<&str> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let access_claims = get_access_claims(ctx);

        profile_service
            .remove_language(rel_type, access_claims.sub().to_string(), lang)
            .await?;

        Ok("OK")
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn edit_lang_level(
        &'a self,
        ctx: &'a Context<'_>,
        lang: Language,
        new_level: CefrKind,
    ) -> GraphQLResult<&str> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let access_claims = get_access_claims(ctx);

        profile_service
            .edit_lang_level(access_claims.sub().to_string(), lang, new_level)
            .await?;

        Ok("OK")
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
    .or(AuthGuard::new(Permission::Developer))
    .or(AuthGuard::new(Permission::User))")]
    async fn edit_profile_info(
        &'a self,
        ctx: &'a Context<'_>,
        input: EditProfileInput,
    ) -> GraphQLResult<&str> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let access_claims = get_access_claims(ctx);

        profile_service
            .edit_profile_props(input, access_claims.sub().to_string())
            .await?;

        Ok("OK")
    }
}

#[derive(Default)]
pub struct ProfileQuery;

#[Object]
impl<'a> ProfileQuery {
    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn get_profile_by_id(
        &'a self,
        ctx: &'a Context<'_>,
        id: String,
    ) -> GraphQLResult<Profile> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;

        Ok(profile_service.get_by_id(id).await?)
    }
}

/// Получение полезной нагрузки Access токена
fn get_access_claims<'a>(ctx: &'a Context<'_>) -> &'a AccessClaims {
    ctx.data_opt::<Result<Option<AccessClaims>, CustomError>>()
        .unwrap()
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap()
}
