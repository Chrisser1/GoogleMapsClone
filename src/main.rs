mod database;
mod node;
mod utils;
mod way;
mod tag;
mod way_node;
mod member;
mod relations;
mod open_street_map;

use database::Database;
use member::Member;
use node::Node;
use open_street_map::read_nodes_from_file;
use relations::Relation;
use tag::Tag;
use way::Way;
use way_node::WayNode;
use std::process;

const BATCH_SIZE: usize = 5000; // Adjust based on your system's capability and the expected dataset size
const CONNECTION_STRING: &str =
    "Driver={ODBC Driver 17 for SQL Server};\
    Server=localhost,1433;UID=sa;\
    PWD=KortKode123;\
    Encrypt=no;TrustServerCertificate=no;\
    Connection Timeout=30;";

fn main() {
    // Create a database instance with the full connection string.
    let database = Database::new(CONNECTION_STRING).unwrap_or_else(|e| {
        eprintln!("Failed to connect to the database: {}", e);
        process::exit(1);
    });

    // Read nodes from file
    let nodes_result = read_nodes_from_file("utils/mapdata/map");

    match nodes_result {
        Ok(nodes) => {
            println!("Inserting nodes");
            // Insert nodes into the database
            match database.insert_node_and_tag(&nodes) {
                Ok(_) => println!("Successfully inserted nodes and tags"),
                Err(e) => {
                    eprintln!("Error inserting nodes and tags: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read nodes from file: {}", e);
            process::exit(1);
        }
    }
    println!("Done with insertion")


    // let ways = vec![
    //     Way {
    //         id: 1,
    //         version: 1,
    //         timestamp: "2024-05-10 12:36:56".to_string(),
    //         changeset: 32,
    //         uid: 23,
    //         user: "username".to_string(),
    //         nodes: vec![
    //             WayNode { way_id: 1, ref_id: 1 }
    //         ],
    //         tags: vec![
    //             Tag { key: "amenity".to_string(), value: "cafe".to_string() },
    //             Tag { key: "open".to_string(), value: "yes".to_string() },
    //         ],
    //     },
    //     Way {
    //         id: 2,
    //         version: 2,
    //         timestamp: "2024-05-11 12:36:56".to_string(),
    //         changeset: 12,
    //         uid: 43,
    //         user: "username".to_string(),
    //         nodes: vec![
    //             WayNode { way_id: 2, ref_id: 2 }
    //         ],
    //         tags: vec![
    //             Tag { key: "amenity".to_string(), value: "library".to_string() },
    //             Tag { key: "open".to_string(), value: "yes".to_string() },
    //             Tag { key: "floors".to_string(), value: "2".to_string() },
    //         ],
    //     }
    // ];

    // let relations = vec![
    //     Relation {
    //         id: 1,
    //         version: 234,
    //         timestamp: "Hello Lukas".to_string(),
    //         changeset: 123,
    //         uid: 321321,
    //         user: "Lukas".to_string(),
    //         member: vec![
    //             Member {
    //                 id: 1,
    //                 ref_id: 1,
    //                 maps_type: utils::MapsType::Way,
    //                 role: "".to_string()
    //             }
    //         ],
    //         tags: vec![
    //             Tag { key: "amenity".to_string(), value: "library".to_string() },
    //             Tag { key: "open".to_string(), value: "yes".to_string() },
    //             Tag { key: "floors".to_string(), value: "2".to_string() },
    //         ],
    //     },
    //     Relation {
    //         id: 2,
    //         version: 434,
    //         timestamp: "Hello Christoffer".to_string(),
    //         changeset: 321,
    //         uid: 3213,
    //         user: "Christoffer".to_string(),
    //         member: vec![
    //             Member {
    //                 id: 2,
    //                 ref_id: 1,
    //                 maps_type: utils::MapsType::Relation,
    //                 role: "".to_string()
    //             },

    //         ],
    //         tags: vec![
    //             Tag { key: "amenity".to_string(), value: "library".to_string() },
    //             Tag { key: "open".to_string(), value: "yes".to_string() },
    //             Tag { key: "floors".to_string(), value: "2".to_string() },
    //         ],
    //     },
    // ];

    // // Try to insert nodes
    // match database.insert_node_and_tag(&nodes) {
    //     Ok(_) => println!("----------------"),
    //     Err(e) => {
    //         eprintln!("Error inserting node and tag: {}", e);
    //         process::exit(1);
    //     }
    // }

    // // Try to insert ways
    // match database.inser_way_with_tag_and_way_nodes(&ways) {
    //     Ok(_) => println!("----------------"),
    //     Err(e) => {
    //         eprintln!("Error inserting way with tag and ref: {}", e);
    //         process::exit(1);
    //     }
    // }

    // // Try to insert ways
    // match database.inser_relation_with_tag_and_member(&relations) {
    //     Ok(_) => println!("----------------"),
    //     Err(e) => {
    //         eprintln!("Error inserting way with tag and ref: {}", e);
    //         process::exit(1);
    //     }
    // }

    // // Query nodes with a version greater than 2 to retrieve some of the newly inserted nodes
    // match database.query_nodes(1) {
    //     Ok(nodes) => {
    //         for node in nodes {
    //             println!("Retrieved node with ID: {}, Tags: {:?}", node.id, node.tags);
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Error querying nodes: {}", e);
    //         process::exit(1);
    //     }
    // }
}
