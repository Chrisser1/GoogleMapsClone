mod database;
mod node;
mod utils;
mod way;
mod tag;
mod member;
mod relation;
mod open_street_map;
mod fetcher;

use database::{create_tables, fetch_all_nodes_and_tags, fetch_all_relations_and_tags, fetch_all_ways_and_tags};
use fetcher::read_openstreet_map_file;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use anyhow::Result;

const DB_URL: &str = "sqlite://database/sqlite.db";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Create a database instance with the full connection string.
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await?;
        println!("Database created successfully");
    } else {
        println!("Database already exists");
    }

    let pool = SqlitePool::connect(DB_URL).await?;

    create_tables(&pool).await?;
    println!("Tables created successfully");

    // Read and process the chosen map file
    read_openstreet_map_file(&pool).await?;

    let nodes = match fetch_all_nodes_and_tags(&pool).await {
        Ok(nodes) => nodes,
        Err(error) => panic!("There was a problem fetching the nodes: {:?}", error),
    };

    let ways = match fetch_all_ways_and_tags(&pool).await {
        Ok(ways) => ways,
        Err(error) => panic!("There was a problem fetching the ways: {:?}", error),
    };

    let relations = match fetch_all_relations_and_tags(&pool).await {
            Ok(relations) => relations,
            Err(error) => panic!("There was a problem fetching the relations: {:?}", error),
        };

    println!("Number of nodes: {}", nodes.len());
    println!("Number of ways: {}", ways.len());
    println!("Number of relations: {}", relations.len());
    Ok(())
}
