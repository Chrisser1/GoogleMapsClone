mod database;
mod node;
mod utils;
mod way;
mod tag;
mod member;
mod relation;
mod open_street_map;

use database::{create_tables, fetch_all_nodes_and_tags, fetch_all_relations_and_tags, fetch_all_ways_and_tags, insert_node_data, insert_relation_data, insert_way_data};
use open_street_map::{read_nodes_from_file, read_ways_from_file, read_relations_from_file};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::time::Instant;
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

    // Read nodes from file
    println!("Reading nodes");
    let nodes: Vec<node::Node> = match read_nodes_from_file("utils/mapdata/map") {
        Ok(nodes) => nodes,
        Err(error) => panic!("There was a problem reading the nodes: {:?}", error),
    };
    println!("Read {} nodes", nodes.len());

    // Read ways from file
    println!("Reading ways");
    let ways: Vec<way::Way> = match read_ways_from_file("utils/mapdata/map") {
        Ok(ways) => ways,
        Err(error) => panic!("There was a problem reading the ways: {:?}", error),
    };
    println!("Read {} ways", ways.len());

    // Read relations from file
    println!("Reading relations");
    let relations: Vec<relation::Relation> = match read_relations_from_file("utils/mapdata/map") {
        Ok(relations) => relations,
        Err(error) => panic!("There was a problem reading the relations: {:?}", error),
    };
    println!("Read {} relations", relations.len());

    // Measure the time taken to insert the data
    println!("Inserting data");
    let start = Instant::now();
    insert_node_data(&pool, nodes).await?;
    println!("Inserted nodes");
    insert_way_data(&pool, ways).await?;
    println!("Inserted ways");
    insert_relation_data(&pool, relations).await?;
    println!("Inserted relations");
    let duration = start.elapsed();
    println!("Inserted data in {:?}", duration);
    println!("Done with insertion");

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
    // println!("Relations read are: {:#?}", relations);
    Ok(())
}
