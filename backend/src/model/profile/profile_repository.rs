use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use neo4j_cypher::entity::NodeTrait;
use neo4j_cypher::query::{Query, QueryTrait};
use neo4rs::{Graph, Node, RowStream};
use std::sync::Arc;

use crate::app::db::neo4j::NULL;
use crate::model::language::{
    language_model::{CefrKind, Language, Studied},
    language_mutation::StudiedInput,
};
use crate::{app::core::error::CustomError, neo4j_result};

use super::profile_model::Profile;
use super::profile_mutation::EditProfileInput;
use super::profile_node::{NATIVE_SPEAKER, STUDIED};

type EmptyResult<'a> = Result<(), CustomError<'a>>;

#[async_trait]
pub trait ProfileRepositoryT: Send + Sync {
    async fn create(
        &self,
        profile: Arc<Profile>,
        native_langs: Vec<Language>,
        studied_langs: Vec<StudiedInput>,
    ) -> EmptyResult;
    async fn subscribe(&self, to_id: String, from_id: String) -> EmptyResult;
    async fn unsubscribe(&self, profile_id: String, from_id: String) -> EmptyResult;
    async fn remove_language(
        &self,
        rel_type: String,
        profile_id: String,
        lang: Language,
    ) -> EmptyResult;
    async fn edit_lang_level(
        &self,
        profile_id: String,
        lang: Language,
        level: CefrKind,
    ) -> EmptyResult;
    async fn edit_profile_props(&self, input: EditProfileInput, id: String) -> EmptyResult;

    async fn get_data(&self, username: String) -> Result<Profile, CustomError>;
    async fn get_native_langs(&self, find_by: String) -> Result<Vec<Language>, CustomError>;
    async fn get_studied_langs(&self, find_by: String) -> Result<Vec<Studied>, CustomError>;
}

pub struct ProfileRepository {
    neo: Arc<Graph>,
}

impl ProfileRepository {
    pub fn new(neo4j: &Arc<Graph>) -> Self {
        Self { neo: neo4j.clone() }
    }
}

#[async_trait]
impl ProfileRepositoryT for ProfileRepository {
    /* ======================== MUTATIONS ======================== */

    async fn edit_profile_props(&self, input: EditProfileInput, id: String) -> EmptyResult {
        let query = neo4rs::query(&format!(
            "
            MATCH (n:Profile) WHERE n.id = $id

            SET n.username = $username
            SET n.first_name = $first_name
            SET n.last_name = {last_name}
            SET n.description = {description}

            RETURN n
            ",
            last_name = if input.last_name.is_some() {
                format!("'{}'", input.last_name.unwrap())
            } else {
                NULL.to_string()
            },
            description = if input.description.is_some() {
                format!("'{}'", input.description.unwrap())
            } else {
                NULL.to_string()
            }
        ))
        .param("id", id)
        .param("username", input.username)
        .param("first_name", input.first_name);

        neo4j_result!(self.neo.run(query).await)?;
        Ok(())
    }

    /// Обновить CEFR поле в связи узла :Profile и :Language
    async fn edit_lang_level(
        &self,
        profile_id: String,
        lang: Language,
        level: CefrKind,
    ) -> EmptyResult {
        let query = neo4rs::query(
            "
            MATCH (p:Profile)-[r:STUDIED]->(l:Language)
            WHERE p.id = $id AND l.name = $name
            SET r.cefr = $level
            ",
        )
        .param("id", profile_id)
        .param("name", lang.to_string())
        .param("level", level.to_string());

        neo4j_result!(self.neo.run(query).await)?;
        Ok(())
    }

    /// Удалить связь нужного типа с конкретным языком
    async fn remove_language(
        &self,
        rel_type: String,
        profile_id: String,
        lang_name: Language,
    ) -> EmptyResult {
        match rel_type.to_uppercase().as_str() {
            NATIVE_SPEAKER | STUDIED => {
                let query = neo4rs::query(&format!(
                    "
                        MATCH (p:Profile)-[r:{}]->(l:Language)
                        WHERE p.id = $id AND l.name = $name
                        DELETE r
                    ",
                    rel_type.to_uppercase()
                ))
                .param("id", profile_id)
                .param("name", lang_name.to_string());

                neo4j_result!(self.neo.run(query).await)?;
                Ok(())
            }

            _ => Err(crate::unprocessable!("relationship", None)),
        }
    }

    /// Удалить связь `:SUBSCRIBE` с указанным пользователем
    async fn unsubscribe(&self, profile_id: String, from_id: String) -> EmptyResult {
        let query = neo4rs::query(
            "
                MATCH (p1:Profile)-[r:SUBSCRIBE]->(p2:Profile)
                WHERE p1.id = $id AND p2.id = $from_id
                DELETE r
            ",
        )
        .param("id", profile_id)
        .param("from_id", from_id);

        neo4j_result!(self.neo.run(query).await)?;
        Ok(())
    }

    /// Создать связь `:SUBSCRIBE` с указанным пользователем
    async fn subscribe(&self, to_id: String, from_id: String) -> EmptyResult {
        let query = neo4rs::query(
            "
                MATCH (e:Profile) WHERE e.id = $form
                MATCH (d:Profile) WHERE d.id = $to

                CREATE (e)-[:SUBSCRIBE {timestamp: $timestamp}]->(d)
            ",
        )
        .param("form", from_id)
        .param("to", to_id)
        .param("timestamp", Utc::now().timestamp());

        neo4j_result!(self.neo.run(query).await)?;
        Ok(())
    }

    async fn create(
        &self,
        profile: Arc<Profile>,
        native_langs: Vec<Language>,
        studied_langs: Vec<StudiedInput>,
    ) -> EmptyResult {
        let txn = self.neo.start_txn().await?;

        neo4j_result!(
            txn.run_queries(vec![
                create_user_query(&profile),
                set_relationships_query(profile.id.to_string(), native_langs, studied_langs)
            ])
            .await
        )?;

        Ok(neo4j_result!(txn.commit().await)?)
    }

    /* ======================== QUERYS ======================== */

    async fn get_data(&self, find_by: String) -> Result<Profile, CustomError> {
        let query = neo4rs::query(
            "MATCH (n:Profile)
            WHERE n.id = $value OR n.username = $value OR n.email = $value
            RETURN n",
        )
        .param("value", find_by);
        let result = neo4j_result!(self.neo.execute(query).await)?;

        Ok(get_user_query(result).await?)
    }

    async fn get_native_langs(&self, find_by: String) -> Result<Vec<Language>, CustomError> {
        let query = neo4rs::query(
            "MATCH (n:Profile)-[r:NATIVE_SPEAKER]-(l)
            WHERE n.id = $value OR n.username = $value OR n.email = $value
            RETURN r, n, l",
        )
        .param("value", find_by);

        let mut result = neo4j_result!(self.neo.execute(query).await)?;
        let mut output: Vec<Language> = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let Some(_) = row.get::<neo4rs::Relation>("r") {
                // Получаю информацию о языковом узле
                let node = row.get::<neo4rs::Node>("l").unwrap();

                output.push(
                    Language::try_from(node.get::<String>("code").unwrap().as_str()).unwrap(),
                );
            }
        }

        Ok(output)
    }

    async fn get_studied_langs(&self, find_by: String) -> Result<Vec<Studied>, CustomError> {
        let query = neo4rs::query(
            "MATCH (n:Profile)-[r:STUDIED]-(l)
            WHERE n.id = $value OR n.username = $value OR n.email = $value
            RETURN r, n, l",
        )
        .param("value", find_by);

        let mut result = neo4j_result!(self.neo.execute(query).await)?;
        let mut output: Vec<Studied> = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let Some(rel) = row.get::<neo4rs::Relation>("r") {
                // Получаю информацию о языковом узле
                let node = row.get::<neo4rs::Node>("l").unwrap();

                output.push(Studied::new(
                    CefrKind::try_from(rel.get::<String>("cefr").unwrap().as_str()).unwrap(),
                    Language::try_from(node.get::<String>("code").unwrap().as_str()).unwrap(),
                ));
            }
        }

        Ok(output)
    }
}

async fn get_user_query<'a>(mut result: RowStream) -> Result<Profile, CustomError<'a>> {
    let mut pnode: Option<Node> = None;

    while let Ok(Some(row)) = result.next().await {
        pnode = row.get::<neo4rs::Node>("n");
    }

    if let Some(pnode) = pnode {
        Ok(Profile::parse_query_resp(pnode)?)
    } else {
        Err(crate::not_found!("user"))
    }
}

/// Вспомогательная функция для формирования запроса на создание узла пользователя
fn create_user_query(profile: &Arc<Profile>) -> neo4rs::Query {
    neo4rs::query(
        &Query::init()
            .create(vec![&profile.node("n").into()])
            .finalize(),
    )
}

/// Вспомогательная функция в отвечающая за создание связей с языковыми узлами
///
/// Связь с языковыми узлами устанавливается двух типов:
/// 1. NATIVE_SPEAKER (носитель языка)
/// 2. STUDIED (изучение зыка)
///
/// Связь типа NATIVE_SPEAKER не содержит параметров.
/// Связь типа STUDIED а параметрах содержит параметр прогресса в изучении.
/// Прогресс в изучении поределяется по шкале CEFR.
fn set_relationships_query(
    profile_id: String,
    native_langs: Vec<Language>,
    studied_langs: Vec<StudiedInput>,
) -> neo4rs::Query {
    // Получение пользователя которому необходимо установить отношения
    let mut base = format!("MATCH (p:Profile) WHERE p.id = '{}'\n", profile_id);
    let mut create_query = String::from("\n\nCREATE");

    let (studied_match_query, studied_create_query) = studied_languages_pquery(
        &studied_langs
            .into_iter()
            .map(|language| language.into())
            .collect::<Vec<Studied>>(),
    );
    let (native_match_query, native_create_query) = native_languages_pquery(&native_langs);

    // Объединение запросов создания связей
    create_query.push_str(&studied_create_query);
    create_query.push_str(&native_create_query);
    // Удаление последнего символа
    create_query.pop();

    // Объединение в финальный запрос
    base.push_str(&studied_match_query);
    base.push_str(&native_match_query);
    base.push_str(&create_query);

    neo4rs::Query::new(base)
}

fn studied_languages_pquery(studied_languages: &Vec<Studied>) -> (String, String) {
    let mut create_query = String::new();

    // Получение запрашиваемых языковых узлов
    let match_query = studied_languages
        .iter()
        .map(|item| {
            // Формирование запроса на создание связи
            create_query.push_str(&format!(
                " (p)-[:STUDIED {{cefr: '{}' }}]->(language_{}),",
                item.cefr, item.lang
            ));

            // Формирование запроса для получения нужного языкового узла
            format!(
                "\nMATCH (language_{}:Language) WHERE language_{}.code = '{}'",
                item.lang, item.lang, item.lang
            )
        })
        .collect::<String>();

    (match_query, create_query)
}

fn native_languages_pquery(native_languages: &Vec<Language>) -> (String, String) {
    let mut create_query = String::new();

    // Получение запрашиваемых языковых узлов
    let match_query = native_languages
        .iter()
        .map(|item| {
            // Формирование запроса на создание связи
            create_query.push_str(&format!(
                " (p)-[:NATIVE_SPEAKER]->(language_{}),",
                item.to_string()
            ));

            // Формирование запроса для получения нужного языкового узла
            format!(
                "\nMATCH (language_{}:Language) WHERE language_{}.code = '{}'",
                item.to_string(),
                item.to_string(),
                item.to_string()
            )
        })
        .collect::<String>();

    (match_query, create_query)
}
