use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use neo4rs::{Graph, Node};
use std::sync::Arc;

use crate::app::db::neo4j::NULL;
use crate::model::language::language_model::{Language, StudLang};
use crate::{app::core::error::CustomError, neo4j_result};

use super::profile_model::Profile;

#[async_trait]
pub trait ProfileRepositoryT: Send + Sync {
    async fn create(&self, profile: Arc<Profile>) -> Result<(), CustomError>;
    async fn get_by_username(&self, username: String) -> Result<Profile, CustomError>;
    async fn subscribe(&self, to_id: String, from_id: String) -> Result<(), CustomError>;
    async fn get_by_id(&self, id: String) -> Result<Profile, CustomError>;
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
    async fn get_by_id(&self, id: String) -> Result<Profile, CustomError> {
        use crate::app::core::error::CustomErrorKind::{Internal, NotFound};

        let query = neo4rs::query(
            "
        MATCH (n:Profile {id: $id})-[r]-(l)
        RETURN r, l
        ",
        )
        .param("id", id);
        let mut result = neo4j_result!(self.neo.execute(query).await)?;


        println!("{:#?}", result.next().await.unwrap());

        match neo4j_result!(result.next().await)? {
            Some(row) => {
                // println!("{:#?}", row);

                let node = row.get::<Node>("p").unwrap();

                match Profile::from_node(node) {
                    Ok(profile) => Ok(profile),
                    Err(error) => Err(CustomError::new()
                        .kind(Internal)
                        .details(error.to_string().as_str())
                        .build()),
                }
            }
            None => Err(CustomError::new().kind(NotFound).build()),
        }
    }

    async fn subscribe(&self, to_id: String, from_id: String) -> Result<(), CustomError> {
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

    async fn get_by_username(&self, username: String) -> Result<Profile, CustomError> {
        use crate::app::core::error::CustomErrorKind::{Internal, NotFound};

        let query = neo4rs::query("MATCH (p:Profile {username: $username}) RETURN p")
            .param("username", username);
        let mut result = self.neo.execute(query).await.expect("Faild to get record");

        match neo4j_result!(result.next().await)? {
            Some(row) => {
                let node = row.get::<Node>("p").unwrap();

                match Profile::from_node(node) {
                    Ok(profile) => Ok(profile),
                    Err(error) => Err(CustomError::new()
                        .kind(Internal)
                        .details(error.to_string().as_str())
                        .build()),
                }
            }

            None => Err(CustomError::new().kind(NotFound).build()),
        }
    }

    async fn create(&self, profile: Arc<Profile>) -> Result<(), CustomError> {
        let txn = self.neo.start_txn().await?;

        neo4j_result!(
            txn.run_queries(vec![
                create_user_query(&profile),
                set_relationships_query(&profile)
            ])
            .await
        )?;

        neo4j_result!(txn.commit().await)?;

        Ok(())
    }
}

/// Вспомогательная функция для формирования запроса на создание узла пользователя
fn create_user_query(profile: &Arc<Profile>) -> neo4rs::Query {
    neo4rs::query(&format!(
        "CREATE (p:Profile {{
            id: $id, 
            email: $email, 
            hash: $hash,            
            username: $username, 
            first_name: $first_name, 
            last_name: {last_name}, 
            sex: $sex, 
            age: $age, 
            description: {description}, 
            created_at: $created_at, 
            updated_at: $updated_at
        }})

        SET p:User
    ",
        last_name = if profile.last_name.is_some() {
            profile.last_name.as_ref().unwrap()
        } else {
            NULL
        },
        description = if profile.description.is_some() {
            profile.description.as_ref().unwrap()
        } else {
            NULL
        }
    ))
    .param("id", profile.id.to_string())
    .param("email", profile.email.to_string())
    .param("hash", profile.hash.to_string())
    .param("username", profile.username.to_string())
    .param("first_name", profile.first_name.to_string())
    .param("sex", profile.sex as i64)
    .param("age", profile.age as i64)
    .param("created_at", profile.created_at)
    .param("updated_at", profile.updated_at)
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
fn set_relationships_query(profile: &Arc<Profile>) -> neo4rs::Query {
    // Получение пользователя которому необходимо установить отношения
    let mut base = format!(
        "MATCH (p:Profile) WHERE p.id = '{}'\n",
        profile.id.to_string()
    );
    let mut create_query = String::from("\n\nCREATE");

    let (studied_match_query, studied_create_query) =
        studied_languages_pquery(&profile.studied_languages);
    let (native_match_query, native_create_query) =
        native_languages_pquery(&profile.native_languages);

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

fn studied_languages_pquery(studied_languages: &Vec<StudLang>) -> (String, String) {
    let mut create_query = String::new();

    // Получение запрашиваемых языковых узлов
    let match_query = studied_languages
        .iter()
        .map(|item| {
            // Формирование запроса на создание связи
            create_query.push_str(&format!(" (p)-[:STUDIED]->(language_{}),", item.lang));

            // Формирование запроса для получения нужного языкового узла
            format!(
                "\nMATCH (language_{}:Language) WHERE language_{}.name = '{}'",
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
                "\nMATCH (language_{}:Language) WHERE language_{}.name = '{}'",
                item.to_string(),
                item.to_string(),
                item.to_string()
            )
        })
        .collect::<String>();

    (match_query, create_query)
}
