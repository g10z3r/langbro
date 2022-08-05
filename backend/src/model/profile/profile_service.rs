use anyhow::Result;
use async_trait::async_trait;
use neo4rs::Graph;
use std::sync::Arc;

use crate::{app::core::error::CustomError, neo4j_result};

use super::profile_model::Profile;

#[async_trait]
pub trait ProfileServiceT: Send + Sync {
    async fn create(&self, profile: Profile) -> Result<(), CustomError>;
}

pub struct ProfileService {
    neo: Arc<Graph>,
}

impl ProfileService {
    pub fn new(neo4j: &Arc<Graph>) -> Self {
        Self { neo: neo4j.clone() }
    }
}

#[async_trait]
impl ProfileServiceT for ProfileService {
    async fn create(&self, profile: Profile) -> Result<(), CustomError> {
        let txn = self.neo.start_txn().await?;

        let mut native_language_query_base = String::new();
        let mut native_language_query_match =
            String::from(format!("MATCH (p:Person) WHERE p.id = $id",));

        let mut native_language_query_create = String::from("\n\nCREATE ");
        for i in profile.native_languages.iter() {
            // Выбираю все узлы необходимых языков
            native_language_query_match.push_str(&format!(
                "\nMATCH (language_{}:Language) WHERE language_{}.name = '{}'",
                i.to_string(),
                i.to_string(),
                i.to_string()
            ));

            // Создаю связи между пользователями и выбранными узлами языков
            native_language_query_create.push_str(&format!(
                "(p)-[:NATIVE_SPEAKER]->(language_{}),",
                i.to_string(),
            ))
        }

        // Убираю последнею запятую
        native_language_query_create.pop();

        // Склеиваю все в один запрос
        native_language_query_base.push_str(&native_language_query_match);
        native_language_query_base.push_str(&native_language_query_create);

        // Создание финального запроса для установки связей межде пользователем и его родными языками
        let native_language_query_base =
            neo4rs::query(&native_language_query_base).param("id", profile.id.to_string());

        let base_query = neo4rs::query(
            "CREATE (p:Person {
            id: $id, 
            email: $email, 
            hash: $hash,            
            username: $username, 
            first_name: $first_name, 
            last_name: NULL, 
            sex: $sex, 
            age: $age, 
            description: $description, 
            created_at: $created_at, 
            updated_at: $updated_at
        })
        
        SET p:User
        ",
        )
        .param("id", profile.id.to_string())
        .param("email", profile.email)
        .param("hash", profile.hash)
        .param("username", profile.username)
        .param("first_name", profile.first_name)
        .param("last_name", profile.last_name.unwrap_or("".to_string()))
        .param("sex", profile.sex as i64)
        .param("age", profile.age as i64)
        .param("description", profile.description.unwrap_or("".to_string()))
        .param("created_at", profile.created_at)
        .param("updated_at", profile.updated_at);

        neo4j_result!(
            txn.run_queries(vec![base_query, native_language_query_base])
                .await
        )?;

        neo4j_result!(txn.commit().await)?;

        Ok(())
    }
}
