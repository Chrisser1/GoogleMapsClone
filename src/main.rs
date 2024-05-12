mod database;
mod node;
mod utils;
mod way;
mod tag;
mod way_node;

use database::Database;
use node::Node;
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

    // Define the sample nodes with tags
    let nodes = vec![
        Node {
            id: 1,
            lat: 52.5200,
            lon: 13.4050,
            version: 1,
            timestamp: "2024-05-10 12:34:56".to_string(),
            changeset: 100,
            uid: 42,
            user: "username".to_string(),
            tags: vec![
                Tag { key: "amenity".to_string(), value: "cafe".to_string() },
                Tag { key: "open".to_string(), value: "yes".to_string() },
            ],
        },
        Node {
            id: 2,
            lat: 52.5200,
            lon: 14.4050,
            version: 2,
            timestamp: "2024-05-10 12:36:56".to_string(),
            changeset: 120,
            uid: 41,
            user: "username".to_string(),
            tags: vec![
                Tag { key: "amenity".to_string(), value: "library".to_string() },
                Tag { key: "open".to_string(), value: "yes".to_string() },
                Tag { key: "floors".to_string(), value: "2".to_string() },
            ],
        }
    ];

    let ways = vec![
        Way {
            id: 1,
            version: 1,
            timestamp: "2024-05-10 12:36:56".to_string(),
            changeset: 32,
            uid: 23,
            user: "username".to_string(),
            nodes: vec![
                WayNode { way_id: 1, ref_id: 1 }
            ],
            tags: vec![
                Tag { key: "amenity".to_string(), value: "cafe".to_string() },
                Tag { key: "open".to_string(), value: "yes".to_string() },
            ],
        },
        Way {
            id: 2,
            version: 2,
            timestamp: "2024-05-11 12:36:56".to_string(),
            changeset: 12,
            uid: 43,
            user: "username".to_string(),
            nodes: vec![
                WayNode { way_id: 2, ref_id: 2 }
            ],
            tags: vec![
                Tag { key: "amenity".to_string(), value: "library".to_string() },
                Tag { key: "open".to_string(), value: "yes".to_string() },
                Tag { key: "floors".to_string(), value: "2".to_string() },
            ],
        }
    ];

    // // Try to insert a node and a tag
    // match database.insert_node_and_tag(&nodes) {
    //     Ok(_) => println!("Inserted successfully."),
    //     Err(e) => {
    //         eprintln!("Error inserting node and tag: {}", e);
    //         process::exit(1);
    //     }
    // }

    // // Try to insert a node and a tag
    // match database.inser_way_with_tag_and_way_nodes(&ways) {
    //     Ok(_) => println!("Node and tag inserted successfully."),
    //     Err(e) => {
    //         eprintln!("Error inserting way with tag and ref: {}", e);
    //         process::exit(1);
    //     }
    // }

    // Query nodes with a version greater than 2 to retrieve some of the newly inserted nodes
    match database.query_nodes(1) {
        Ok(nodes) => {
            for node in nodes {
                println!("Retrieved node with ID: {}, Tags: {:?}", node.id, node.tags);
            }
        },
        Err(e) => {
            eprintln!("Error querying nodes: {}", e);
            process::exit(1);
        }
    }
}
