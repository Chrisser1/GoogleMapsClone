use std::fs;
use std::io::{self, Write};
use std::time::Instant;
use sqlx::SqlitePool;
use anyhow::Result;

use crate::database::{insert_node_data, insert_relation_data, insert_way_data};
use crate::{node, relation, way};
use crate::open_street_map::{read_nodes_from_file, read_relations_from_file, read_ways_from_file};

fn list_files_in_directory(directory: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    files.push(file_name_str.to_string());
                }
            }
        }
    }
    Ok(files)
}

fn choose_file(files: &[String]) -> Option<String> {
    println!("Available map files:");
    for (index, file) in files.iter().enumerate() {
        println!("{}: {}", index + 1, file);
    }
    println!("Please enter the number of the file you want to choose:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= files.len() => Some(files[index - 1].clone()),
        _ => None,
    }
}

async fn process_map_file(pool: &SqlitePool, file_path: &str) -> Result<()> {
    let full_path = format!("utils/mapdata/{}", file_path);

    // Read nodes from file
    println!("Reading data");
    let start = Instant::now();
    println!("Reading nodes");
    let nodes: Vec<node::Node> = match read_nodes_from_file(&full_path) {
        Ok(nodes) => nodes,
        Err(error) => panic!("There was a problem reading the nodes: {:?}", error),
    };
    println!("Read {} nodes", nodes.len());

    // Read ways from file
    println!("Reading ways");
    let ways: Vec<way::Way> = match read_ways_from_file(&full_path) {
        Ok(ways) => ways,
        Err(error) => panic!("There was a problem reading the ways: {:?}", error),
    };
    println!("Read {} ways", ways.len());

    // Read relations from file
    println!("Reading relations");
    let relations: Vec<relation::Relation> = match read_relations_from_file(&full_path) {
        Ok(relations) => relations,
        Err(error) => panic!("There was a problem reading the relations: {:?}", error),
    };
    println!("Read {} relations", relations.len());
    let duration = start.elapsed();
    println!("Read data in {:?}", duration);

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

    Ok(())
}

pub async fn read_openstreet_map_file(pool: &SqlitePool) -> Result<()> {
    let directory = "utils/mapdata/";
    let files = list_files_in_directory(directory)?;

    if let Some(chosen_file) = choose_file(&files) {
        process_map_file(pool, &chosen_file).await?;
    } else {
        println!("Invalid selection.");
    }

    Ok(())
}
