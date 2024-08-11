mod database;
mod osm_entities;
mod utils;
mod open_street_map;
mod fetcher;
mod app;
mod texture;

use app::run;
use database::{create_tables, fetch_all_nodes_and_tags, fetch_all_relations_and_tags, fetch_all_ways_and_tags};
use fetcher::read_openstreet_map_file;

use anyhow::Result;

const DB_URL: &str = "sqlite://database/sqlite.db";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    run().await;
    

    // // Read and process the chosen map file
    // read_openstreet_map_file(&pool).await?;

    // let nodes = match fetch_all_nodes_and_tags(&pool).await {
    //     Ok(nodes) => nodes,
    //     Err(error) => panic!("There was a problem fetching the nodes: {:?}", error),
    // };

    // let ways = match fetch_all_ways_and_tags(&pool).await {
    //     Ok(ways) => ways,
    //     Err(error) => panic!("There was a problem fetching the ways: {:?}", error),
    // };

    // let relations = match fetch_all_relations_and_tags(&pool).await {
    //         Ok(relations) => relations,
    //         Err(error) => panic!("There was a problem fetching the relations: {:?}", error),
    //     };

    // println!("Number of nodes: {}", nodes.len());
    // println!("Number of ways: {}", ways.len());
    // println!("Number of relations: {}", relations.len());
    Ok(())
}
