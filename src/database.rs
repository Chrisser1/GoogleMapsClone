use odbc_api::{
    buffers::{self, BufferDesc, TextRowSet},
    Connection, ConnectionOptions, Cursor, Environment, Error, IntoParameter
};
use std::{process::{self, id}, result::Result};

use crate::{node::{extract_node_changesets, extract_node_ids, extract_node_lats, extract_node_lons, extract_node_timestamps, extract_node_uids, extract_node_users, extract_node_versions, Node}, utils::{parse_f64, parse_i32, parse_i64, parse_string, NodeQueryError}, BATCH_SIZE};

/// `Database` holds the ODBC environment and connection configuration.
/// Structure representing the database connectivity configuration.
pub struct Database {
    env: Environment,
    connection_string: String,
}

impl Database {
    /// Constructs a new `Database`.
    ///
    /// # Arguments
    /// * `connection_string` - A full ODBC connection string.
    ///
    /// # Returns
    /// A result that, if Ok, contains the `Database` instance, or if Err, contains an error.
    pub fn new(connection_string: &str) -> Result<Self, Error> {
        let env = Environment::new()?;
        Ok(Self {
            env,
            connection_string: connection_string.to_owned(),
        })
    }

    /// Gets a database connection using the stored environment and connection details.
    ///
    /// # Returns
    /// A result that, if Ok, contains the `Connection`, or if Err, contains an error.
    pub fn get_connection(&self) -> Result<Connection, Error> {
        self.env.connect_with_connection_string(&self.connection_string, ConnectionOptions::default())
    }

    /// Inserts multiple nodes and their associated tags into the database within a single transaction.
    ///
    /// # Arguments
    /// * `nodes` - A slice of `Node` data to insert.
    /// * `keys` - Keys of the tags.
    /// * `values` - Values of the tags.
    ///
    /// # Returns
    /// A result that, if Ok, signifies successful insertion of all nodes and tags, or if Err, contains an error.
    pub fn insert_node_and_tag(
        &self,
        nodes: &[Node],
        keys: &[&str],
        values: &[&str],
    ) -> Result<(), NodeQueryError> {
        let conn = self.get_connection()?;

        // Try to insert a nodes
        match self.insert_node(&conn, nodes) {
            Ok(_) => println!("Nodes/note inserted successfully."),
            Err(e) => {
                eprintln!("Error inserting node: {}", e);
                process::exit(1);
            }
        }

        // Get node ids slice
        let node_id_refs: Vec<&i64> = extract_node_ids(nodes);
        let node_id_slice: Vec<i64> = node_id_refs.iter().map(|&&id| id).collect();
        let node_ids: &[i64] = &node_id_slice;

        // Try to insert a tag to the node
        match self.insert_tag(&conn, node_ids, keys, values) {
            Ok(_) => println!("Tag inserted successfully."),
            Err(e) => {
                eprintln!("Error inserting tag: {}", e);
                process::exit(1);
            }
        }
        Ok(())
    }

    /// Inserts a node or nodes into the database.
    ///
    /// # Arguments
    /// * `nodes` - The node to be inserted.

    /// # Returns
    /// A result that, if Ok, signifies successful insertion, or if Err, contains an error.
    fn insert_node(
        &self,
        conn: &Connection,
        nodes: &[Node]
    ) -> Result<(), odbc_api::Error> {
        let sql = format!(
            "INSERT INTO [DenmarkMapsDB].[dbo].[nodes] (id, lat, lon, version, timestamp, changeset, uid, [user]) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        );
        // Connect to the database
        let prepared = conn.prepare(&sql)?;

        // Build buffer description
        let buffer_description = [
            BufferDesc::I64 { nullable: false },   // id
            BufferDesc::F64 { nullable: false },   // lat
            BufferDesc::F64 { nullable: false },   // lon
            BufferDesc::I32 { nullable: false },   // version
            BufferDesc::Text { max_str_len: 32 },  // timestamp
            BufferDesc::I64 { nullable: false },   // changeset
            BufferDesc::I64 { nullable: false },   // uid
            BufferDesc::Text { max_str_len: 128 }, // user
        ];

        let mut inserter = prepared.into_column_inserter(nodes.len(), buffer_description)?;
        inserter.set_num_rows(nodes.len());

        ///// Fill the buffer with values column by column ////
        // Id insertion
        let col = inserter
            .column_mut(0)
            .as_slice::<i64>()
            .expect("We know the id column to hold i64.");
        let node_id_refs: Vec<&i64> = extract_node_ids(nodes);
        let node_id_slice: Vec<i64> = node_id_refs.iter().map(|&&id| id).collect();
        let node_id_borrowed_slice: &[i64] = &node_id_slice;
        col.copy_from_slice(node_id_borrowed_slice);

        // Lat insertion
        let col = inserter
            .column_mut(1)
            .as_slice::<f64>()
            .expect("We know the lat column to hold f64.");
        let node_lat_refs: Vec<&f64> = extract_node_lats(nodes);
        let node_lat_slice: Vec<f64> = node_lat_refs.iter().map(|&&id| id).collect();
        let node_lat_borrowed_slice: &[f64] = &node_lat_slice;
        col.copy_from_slice(node_lat_borrowed_slice);

        // lon insertion
        let col = inserter
            .column_mut(2)
            .as_slice::<f64>()
            .expect("We know the lon column to hold f64.");
        let node_lon_refs: Vec<&f64> = extract_node_lons(nodes);
        let node_lon_slice: Vec<f64> = node_lon_refs.iter().map(|&&id| id).collect();
        let node_lon_borrowed_slice: &[f64] = &node_lon_slice;
        col.copy_from_slice(node_lon_borrowed_slice);

        // version insertion
        let col = inserter
            .column_mut(3)
            .as_slice::<i32>()
            .expect("We know the version column to hold i32.");
        let node_version_refs: Vec<&i32> = extract_node_versions(nodes);
        let node_version_slice: Vec<i32> = node_version_refs.iter().map(|&&version| version).collect();
        let node_version_borrowed_slice: &[i32] = &node_version_slice;
        col.copy_from_slice(node_version_borrowed_slice);

        // timestamp insertion
        let mut col = inserter
            .column_mut(4)
            .as_text_view()
            .expect("We know the timestamp column to hold text as time dates.");
        let node_timestamp_refs: Vec<&String> = extract_node_timestamps(nodes);
        let node_timestamp_slices: Vec<&str> = node_timestamp_refs.iter().map(|&ts| ts.as_str()).collect();
        let node_timestamp_slice: &[&str] = &node_timestamp_slices;

        for (index, timestamp) in node_timestamp_slice.iter().enumerate() {
            col.set_cell(index, Some(timestamp.as_bytes()));
        }

        // changeset insertion
        let col = inserter
            .column_mut(5)
            .as_slice::<i64>()
            .expect("We know the changeset column to hold i64.");
        let node_changeset_refs: Vec<&i64> = extract_node_changesets(nodes);
        let node_changeset_slice: Vec<i64> = node_changeset_refs.iter().map(|&&changeset| changeset).collect();
        let node_changeset_borrowed_slice: &[i64] = &node_changeset_slice;
        col.copy_from_slice(node_changeset_borrowed_slice);

        // uid insertion
        let col = inserter
            .column_mut(6)
            .as_slice::<i64>()
            .expect("We know the uid column to hold i64.");
        let node_uid_refs: Vec<&i64> = extract_node_uids(nodes);
        let node_uid_slice: Vec<i64> = node_uid_refs.iter().map(|&&uid| uid).collect();
        let node_uid_borrowed_slice: &[i64] = &node_uid_slice;
        col.copy_from_slice(node_uid_borrowed_slice);

        // user insertion
        let mut col = inserter
            .column_mut(7)
            .as_text_view()
            .expect("We know the user column to hold text as time dates.");
        let node_user_refs: Vec<&String> = extract_node_users(nodes);
        let node_user_slices: Vec<&str> = node_user_refs.iter().map(|&ts| ts.as_str()).collect();
        let node_user_slice: &[&str] = &node_user_slices;

        for (index, user) in node_user_slice.iter().enumerate() {
            col.set_cell(index, Some(user.as_bytes()));
        }

        inserter.execute()?;
        Ok(())
    }

    /// Inserts a tag or tags associated with a node or nodes into the database.
    ///
    /// # Arguments
    /// * `node_ids` - Identifier of the node to which the tag is associated.
    /// * `keys` - Key of the tag/tags.
    /// * `values` - Value of the tag/tags.
    ///
    /// # Returns
    /// A result that, if Ok, signifies successful insertion, or if Err, contains an error.
    fn insert_tag(
        &self,
        conn: &Connection,
        node_ids: &[i64],
        keys: &[&str],
        values: &[&str]
    ) -> Result<(), odbc_api::Error> {
        let sql = format!(
            "INSERT INTO [DenmarkMapsDB].[dbo].[tags] (node_id, [key], value) VALUES (?, ?, ?);",
        );
        // Connect to the database
        let prepared = conn.prepare(&sql)?;

        // Build buffer description
        let buffer_description = [
            BufferDesc::I64 { nullable: false },    // node id
            BufferDesc::Text { max_str_len: 128 },  // key
            BufferDesc::Text { max_str_len: 128 },  // value
        ];

        let mut inserter = prepared.into_column_inserter(node_ids.len(), buffer_description)?;
        inserter.set_num_rows(node_ids.len());

        ///// Fill the buffer with values column by column ////
        // node_id insertion
        let col = inserter
            .column_mut(0)
            .as_slice::<i64>()
            .expect("We know the node_id column to hold i64.");
        col.copy_from_slice(node_ids);

        // key insertion
        let mut col = inserter
            .column_mut(1)
            .as_text_view()
            .expect("We know the key column to hold text as time dates.");

        for (index, key) in keys.iter().enumerate() {
            col.set_cell(index, Some(key.as_bytes()));
        }

        // value insertion
        let mut col = inserter
            .column_mut(2)
            .as_text_view()
            .expect("We know the value column to hold text as time dates.");

        for (index, value) in values.iter().enumerate() {
            col.set_cell(index, Some(value.as_bytes()));
        }

        inserter.execute()?;
        Ok(())
    }

    /// Queries nodes from the database where their version number is greater than the specified value
    /// and returns them as a vector of Node structs.
    ///
    /// # Arguments
    /// * `min_version` - The minimum version number to filter the nodes.
    ///
    /// # Returns
    /// A result that, if Ok, contains a vector of Node structs; if Err, contains an ODBC error.
    pub fn query_nodes(&self, min_version: i32) -> Result<Vec<Node>, NodeQueryError> {
        let sql = "SELECT id, lat, lon, version, timestamp, changeset, uid, user FROM nodes WHERE version > ?";

        let mut conn = self.get_connection()?;
        let mut cursor = conn.execute(sql, (&min_version,))?;

        let mut nodes = Vec::new();

        if let Some(mut cursor) = cursor {
            // Define a reasonable batch size to fetch.
            const BATCH_SIZE: usize = 100;  // Adjust based on your expected data size and memory constraints

            // Use schema in cursor to initialize a text buffer large enough to hold the largest
            // possible strings for each column up to an upper limit of 4KiB.
            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
            // Bind the buffer to the cursor. It is now being filled with every call to fetch.
            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            // Iterate over batches
            while let Some(batch) = row_set_cursor.fetch()? {
                // Within a batch, iterate over every row
                for row_index in 0..batch.num_rows() {
                    let node = Node {
                        id: parse_i64(batch.at(row_index, 0))?,
                        lat: parse_f64(batch.at(row_index, 1))?,
                        lon: parse_f64(batch.at(row_index, 2))?,
                        version: parse_i32(batch.at(row_index, 3))?,
                        timestamp: parse_string(batch.at(row_index, 4))?,
                        changeset: parse_i64(batch.at(row_index, 5))?,
                        uid: parse_i64(batch.at(row_index, 6))?,
                        user: parse_string(batch.at(row_index, 7))?,
                    };
                    nodes.push(node);
                }
            }
        }

        Ok(nodes)
    }
}
