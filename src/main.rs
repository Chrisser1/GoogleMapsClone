mod database;
mod node;
mod utils;

use database::Database;
use node::Node;
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

    // Create a sample nodes
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
        },
        // Add more nodes as needed
    ];

    // Tag details
    let keys = vec!["category", "idk"];
    let values = vec!["landmark", "hello"];

    // Try to insert a node and a tag
    match database.insert_node_and_tag(&nodes, &keys, &values) {
        Ok(_) => println!("Node and tag inserted successfully."),
        Err(e) => {
            eprintln!("Error inserting node and tag: {}", e);
            process::exit(1);
        }
    }
}
