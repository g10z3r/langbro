use actix_web::HttpRequest;
use anyhow::Result;
use async_graphql::{Context, Guard, Result as GraphQLResult};
use chrono::{Duration, Timelike, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{de::DeserializeOwned, Serialize};

use crate::app::core::error::CustomError;
use crate::model::profile::profile_model::Permissions;

pub struct AuthGuard(Permissions);

impl AuthGuard {
    pub fn new(p: Permissions) -> Self {
        Self(p)
    }
}

#[async_trait::async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> GraphQLResult<()> {
        use crate::app::core::error::CustomErrorKind::Forbidden;

        match ctx.data_opt::<Result<Option<AccessClaims>, CustomError>>() {
            Some(result) => match result {
                Ok(data) => match data {
                    Some(acsess_claims) if acsess_claims.permission == self.0 => Ok(()),
                    _ => Err(CustomError::new().kind(Forbidden).build().into()),
                },

                Err(err) => Err(CustomError::new()
                    .kind(err.kind())
                    .some_details(err.details())
                    .build()
                    .into()),
            },

            None => Err(CustomError::new().kind(Forbidden).build().into()),
        }
    }
}

fn split_token(header_value: &str) -> Vec<&str> {
    let split = header_value.split(" ");
    split.collect::<Vec<&str>>()
}

pub fn parse_auth(http_request: HttpRequest) -> Result<Option<AccessClaims>, CustomError> {
    match http_request.headers().get("Authorization") {
        Some(token) if split_token(token.to_str()?).len() == 2 => {
            let claims = Token::<AccessClaims>::decode(split_token(token.to_str()?)[1])?;

            Ok(Some(claims))
        }

        Some(_) => {
            use crate::app::core::error::CustomErrorKind::TokenMissing;

            Err(CustomError::new()
                .kind(TokenMissing)
                .details("Faild to parse token")
                .build())
        }

        None => Ok(None),
    }
}

/* JWT */

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    sub: String,
    exp: i64,
    permission: Permissions,
}

impl AccessClaims {
    pub fn new(sub: String, permission: Permissions, d: Duration) -> AccessClaims {
        // Определение скрока пригодности токена
        let exp = Utc::now() + d;

        // Нормализация к временным меткам UNIX
        let exp = exp
            .date()
            .and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);

        AccessClaims {
            sub,
            exp: exp.timestamp(),
            permission,
        }
    }

    pub fn sub(&self) -> &str {
       &self.sub
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RefreshClaims {
    sub: String,
    exp: i64,
}

impl RefreshClaims {
    pub fn new(sub: String, d: Duration) -> Self {
        // Определение скрока пригодности токена
        let exp = Utc::now() + d;

        // Нормализация к временным меткам UNIX
        let exp = exp
            .date()
            .and_hms_milli(exp.hour(), exp.minute(), exp.second(), 0);

        Self {
            sub,
            exp: exp.timestamp(),
        }
    }
}

#[derive(Debug)]
pub struct Token<C>(C)
where
    C: Serialize + DeserializeOwned;

impl<C> Token<C>
where
    C: Serialize + DeserializeOwned,
{
    pub fn encode(claims: C) -> Result<String> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &EncodingKey::from_secret("secret_key".as_bytes()),
        )?;

        Ok(token)
    }

    pub fn decode(token: &str) -> Result<C, jsonwebtoken::errors::Error> {
        match jsonwebtoken::decode::<C>(
            &token,
            &DecodingKey::from_secret("secret_key".as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(err) => Err(err),
        }
    }
}
