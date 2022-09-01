use async_graphql::{Context, Object, Result as GraphQLResult};
use std::sync::Arc;
use validator::Validate;

use crate::app::api::security::auth::{self, AccessClaims, Token};
use crate::app::core::error::CustomError;
use crate::model::language::language_model::Studied;
use crate::model::language::{
    language_error::{ERR_LANG__DUBLICATED, ERR_LANG__UNIQUE},
    language_model::{CefrKind, Language},
    language_mutation::StudiedInput,
};
use crate::model::profile::{
    profile_error::ERR_PROF__SELF_SUBSCRIBE,
    profile_model::{Permission, Profile},
    profile_mutation::{
        EditProfileInput, ProfileLoginInput, ProfileLoginOutput, ProfileRegistrationInput,
    },
    profile_repository::ProfileRepositoryT,
    profile_resolver::auth::AuthGuard,
};

#[derive(Default)]
pub struct ProfileMutation;

#[Object]
impl<'a> ProfileMutation {
    /// Метод регистрации.
    async fn registration(
        &'a self,
        ctx: &'a Context<'_>,
        profile_input: ProfileRegistrationInput,
        #[graphql(validator(min_items = 1, max_items = 2))] native_langs_input: Vec<Language>,
        #[graphql(validator(min_items = 1, max_items = 4))] studied_langs_input: Vec<StudiedInput>,
    ) -> GraphQLResult<&str> {
        reg_validation(&profile_input, &native_langs_input, &studied_langs_input)?;

        let profile = Arc::new(Profile::new(profile_input)?);
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;

        profile_service
            .create(profile, native_langs_input, studied_langs_input)
            .await?;

        Ok("OK")
    }

    /// Метод авторизации.
    /// В ответ клиент должен получить набор токенов.
    async fn login(
        &'a self,
        ctx: &'a Context<'_>,
        login_input: ProfileLoginInput,
    ) -> GraphQLResult<ProfileLoginOutput> {
        login_input.validate()?;

        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        let profile = profile_service.get_data(login_input.username).await?;

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

    /// Метод удаления связи :SUBSCRIBE между двумя узлами типа :Profile
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

    /// Метод установки связи :SUBSCRIBE между двумя узлами типа :Profile
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
            Err(crate::unprocessable!("id", Some(ERR_PROF__SELF_SUBSCRIBE.to_string())).into())
        }
    }

    /// Метод удаления связи между узлом :Profile и :Language.
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
    async fn get_profile_data(
        &'a self,
        ctx: &'a Context<'_>,
        find_by: String,
    ) -> GraphQLResult<Profile> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        Ok(profile_service.get_data(find_by).await?)
    }

    #[graphql(guard = "AuthGuard::new(Permission::Admin)
        .or(AuthGuard::new(Permission::Developer))
        .or(AuthGuard::new(Permission::User))")]
    async fn get_profile_native_langs(
        &'a self,
        ctx: &'a Context<'_>,
        find_by: String,
    ) -> GraphQLResult<Vec<Language>> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        Ok(profile_service.get_native_langs(find_by).await?)
    }

    async fn get_profile_studied_langs(
        &'a self,
        ctx: &'a Context<'_>,
        find_by: String,
    ) -> GraphQLResult<Vec<Studied>> {
        let profile_service = ctx.data::<Arc<dyn ProfileRepositoryT>>()?;
        Ok(profile_service.get_studied_langs(find_by).await?)
    }
}

fn reg_validation<'a>(
    profile_input: &ProfileRegistrationInput,
    native_langs: &Vec<Language>,
    studied_langs: &Vec<StudiedInput>,
) -> Result<(), CustomError<'a>> {
    profile_input.validate()?;

    // Проверка уникальности
    if (1..native_langs.len()).any(|i| native_langs[i..].contains(&native_langs[i - 1])) {
        return Err(crate::unprocessable!(
            "language",
            Some(ERR_LANG__UNIQUE.to_string())
        ));
    }

    let studied_langs_slim = studied_langs
        .iter()
        .map(|item| item.lang)
        .collect::<Vec<Language>>();

    // Проверка уникальности
    if (1..studied_langs_slim.len())
        .any(|i| studied_langs_slim[i..].contains(&studied_langs_slim[i - 1]))
    {
        return Err(crate::unprocessable!(
            "language",
            Some(ERR_LANG__UNIQUE.to_string())
        ));
    }

    // Проверяю что нет дублирования языков в массиве Родных языков
    // и в списке изучаемых.
    for s in studied_langs_slim {
        if native_langs.contains(&s) {
            return Err(crate::unprocessable!(
                "language",
                Some(ERR_LANG__DUBLICATED.to_string())
            ));
        }
    }

    Ok(())
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
