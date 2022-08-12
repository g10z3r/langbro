use anyhow::Result;
use neo4rs::Graph as Neo4jGraphDB;

pub static NULL: &'static str = "NULL";

pub async fn connect() -> Result<Neo4jGraphDB> {
    let config = neo4rs::config()
        .uri(&format!(
            "{}:{}",
            dotenv!("NEO4J_AUTH_HOST"),
            dotenv!("NEO4J_AUTH_PORT")
        ))
        .user(dotenv!("NEO4J_AUTH_USER"))
        .password(dotenv!("NEO4J_AUTH_PASSWORD"))
        .build()
        .expect("Failed to config Neo4j");

    let db = Neo4jGraphDB::connect(config)
        .await
        .expect("Failed to connect to Neo4j");

    Ok(db)
}
