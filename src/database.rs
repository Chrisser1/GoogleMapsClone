use odbc_api::{
    buffers::{self, BufferDesc, TextRowSet},
    Connection, ConnectionOptions, Cursor, Environment, Error
};
use std::{process::{self, id}, result::Result};

use crate::{node::{self, Node, NodeTag}, tag::{Tag, TagType}, utils::{parse_f64, parse_i32, parse_i64, parse_string, NodeQueryError}, way::{self, Way, WayTag}, way_node::{self, WayNode}, BATCH_SIZE};

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
    ///
    /// # Returns
    /// A result that, if Ok, signifies successful insertion of all nodes and tags, or if Err, contains an error.
    pub fn insert_node_and_tag(
        &self,
        nodes: &[Node],
    ) -> Result<(), NodeQueryError> {
        let conn = self.get_connection()?;

        // Try to insert nodes
        match self.insert_node(&conn, nodes) {
            Ok(_) => println!("Nodes inserted successfully."),
            Err(e) => {
                eprintln!("Error inserting node: {}", e);
                process::exit(1);
            }
        }

        // Get node ids and tags in one
        let node_tags = Node::extract_node_tags(nodes);

        // Collect the node IDs, keys, and values using the new function
        let (node_ids, keys, values) = NodeTag::collect_tag_data(&node_tags);

        // Try to insert a tag to the node
        match self.insert_tag(&conn, &node_ids, &keys, &values, TagType::Node) {
            Ok(_) => println!("Tag inserted successfully."),
            Err(e) => {
                eprintln!("Error inserting tag: {}", e);
                process::exit(1);
            }
        }
        Ok(())
    }

    /// Inserts multiple ways, their associated tags and way_nodes into the database within a single transaction.
    ///
    /// # Arguments
    /// * `ways` - A slice of `Way` data to insert.
    ///
    /// # Returns
    /// A result that, if Ok, signifies successful insertion of all ways, tags and way_nodes, or if Err, contains an error.
    pub fn inser_way_with_tag_and_way_nodes(
        &self,
        ways: &[Way],
    ) -> Result<(), NodeQueryError> {
        let conn = self.get_connection()?;

        // Try to insert ways
        match self.insert_way(&conn, ways) {
            Ok(_) => println!("Ways inserted successfully."),
            Err(e) => {
                eprintln!("Error inserting node: {}", e);
                process::exit(1);
            }
        }

        // Get way ids and tags in one
        let way_tags = Way::extract_way_tags(ways);

        // Collect the way IDs, keys, and values using the new function
        let (way_ids, keys, values) = WayTag::collect_tag_data(&way_tags);

        // Try to insert a tag to the node
        match self.insert_tag(&conn, &way_ids, &keys, &values, TagType::Way) {
            Ok(_) => println!("Tag inserted successfully."),
            Err(e) => {
                eprintln!("Error inserting tag: {}", e);
                process::exit(1);
            }
        }

        let way_nodes = Way::extract_way_nodes(ways);
        let way_nodes_borrowed_slice: &[WayNode] = &way_nodes;

        // Try to insert a tag to the node
        match self.insert_way_node(&conn, way_nodes_borrowed_slice) {
            Ok(_) => println!("WayNode inserted successfully."),
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
    /// * `conn` - Connection to the sql database
    /// * `nodes` - The node to be inserted.

    /// # Returns
    /// A result that, if Ok, signifies successful insertion, or if Err, contains an error.
    fn insert_node(
        &self,
        conn: &Connection,
        nodes: &[Node]
    ) -> Result<(), odbc_api::Error> {
        let sql = format!(
            "INSERT INTO [DenmarkMapsDB].[dbo].[node] (id, lat, lon, version, timestamp, changeset, uid, [user]) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        );
        // Connect to the database
        let prepared = conn.prepare(&sql)?;

        // Build buffer description
        let buffer_description = Node::get_node_buffer_descriptor();

        let mut inserter = prepared.into_column_inserter(nodes.len(), buffer_description)?;
        inserter.set_num_rows(nodes.len());

        ///// Fill the buffer with values column by column ////
        // Id insertion
        let col = inserter
            .column_mut(0)
            .as_slice::<i64>()
            .expect("Failed to insert id for node");
        let node_id_refs: Vec<&i64> = Node::extract(nodes, |node| &node.id);
        let node_id_slice: Vec<i64> = node_id_refs.iter().map(|&&id| id).collect();
        let node_id_borrowed_slice: &[i64] = &node_id_slice;
        col.copy_from_slice(node_id_borrowed_slice);

        // Lat insertion
        let col = inserter
            .column_mut(1)
            .as_slice::<f64>()
            .expect("Failed to insert lat for node");
        let node_lat_refs: Vec<&f64> = Node::extract(nodes, |node| &node.lat);
        let node_lat_slice: Vec<f64> = node_lat_refs.iter().map(|&&id| id).collect();
        let node_lat_borrowed_slice: &[f64] = &node_lat_slice;
        col.copy_from_slice(node_lat_borrowed_slice);

        // lon insertion
        let col = inserter
            .column_mut(2)
            .as_slice::<f64>()
            .expect("Failed to insert lon for node");
        let node_lon_refs: Vec<&f64> = Node::extract(nodes, |node| &node.lon);
        let node_lon_slice: Vec<f64> = node_lon_refs.iter().map(|&&id| id).collect();
        let node_lon_borrowed_slice: &[f64] = &node_lon_slice;
        col.copy_from_slice(node_lon_borrowed_slice);

        // version insertion
        let col = inserter
            .column_mut(3)
            .as_slice::<i32>()
            .expect("Failed to insert version for node");
        let node_version_refs: Vec<&i32> =Node::extract(nodes, |node| &node.version);
        let node_version_slice: Vec<i32> = node_version_refs.iter().map(|&&version| version).collect();
        let node_version_borrowed_slice: &[i32] = &node_version_slice;
        col.copy_from_slice(node_version_borrowed_slice);

        // timestamp insertion
        let mut col = inserter
            .column_mut(4)
            .as_text_view()
            .expect("Failed to insert timestamp for node");
        let node_timestamp_refs: Vec<&String> = Node::extract(nodes, |node| &node.timestamp);
        let node_timestamp_slices: Vec<&str> = node_timestamp_refs.iter().map(|&ts| ts.as_str()).collect();
        let node_timestamp_slice: &[&str] = &node_timestamp_slices;

        for (index, timestamp) in node_timestamp_slice.iter().enumerate() {
            col.set_cell(index, Some(timestamp.as_bytes()));
        }

        // changeset insertion
        let col = inserter
            .column_mut(5)
            .as_slice::<i64>()
            .expect("Failed to insert changeset for node");
        let node_changeset_refs: Vec<&i64> = Node::extract(nodes, |node| &node.changeset);
        let node_changeset_slice: Vec<i64> = node_changeset_refs.iter().map(|&&changeset| changeset).collect();
        let node_changeset_borrowed_slice: &[i64] = &node_changeset_slice;
        col.copy_from_slice(node_changeset_borrowed_slice);

        // uid insertion
        let col = inserter
            .column_mut(6)
            .as_slice::<i64>()
            .expect("Failed to insert uid for node");
        let node_uid_refs: Vec<&i64> = Node::extract(nodes, |node| &node.uid);
        let node_uid_slice: Vec<i64> = node_uid_refs.iter().map(|&&uid| uid).collect();
        let node_uid_borrowed_slice: &[i64] = &node_uid_slice;
        col.copy_from_slice(node_uid_borrowed_slice);

        // user insertion
        let mut col = inserter
            .column_mut(7)
            .as_text_view()
            .expect("Failed to insert user for node");
        let node_user_refs: Vec<&String> = Node::extract(nodes, |node| &node.user);
        let node_user_slices: Vec<&str> = node_user_refs.iter().map(|&ts| ts.as_str()).collect();
        let node_user_slice: &[&str] = &node_user_slices;

        for (index, user) in node_user_slice.iter().enumerate() {
            col.set_cell(index, Some(user.as_bytes()));
        }

        inserter.execute()?;
        Ok(())
    }

    /// Inserts a way or ways into the database.
    ///
    /// # Arguments
    /// * `conn` - Connection to the sql database
    /// * `ways` - The way or ways to be inserted.

    /// # Returns
    /// A result that, if Ok, signifies successful insertion, or if Err, contains an error.
    fn insert_way(
        &self,
        conn: &Connection,
        ways: &[Way]
    ) -> Result<(), odbc_api::Error> {
        let sql = format!(
            "INSERT INTO [DenmarkMapsDB].[dbo].[way] (id, version, timestamp, changeset, uid, [user]) VALUES (?, ?, ?, ?, ?, ?)",
        );
        // Connect to the database
        let prepared = conn.prepare(&sql)?;

        // Build buffer description
        let buffer_description = Way::get_way_buffer_descriptor();

        let mut inserter = prepared.into_column_inserter(ways.len(), buffer_description)?;
        inserter.set_num_rows(ways.len());

        ///// Fill the buffer with values column by column ////
        // Id insertion
        let col = inserter
            .column_mut(0)
            .as_slice::<i64>()
            .expect("Failed to insert id for way");
        let way_id_refs: Vec<&i64> = Way::extract(ways, |way| &way.id);
        let way_id_slice: Vec<i64> = way_id_refs.iter().map(|&&id| id).collect();
        let way_id_borrowed_slice: &[i64] = &way_id_slice;
        col.copy_from_slice(way_id_borrowed_slice);

        // version insertion
        let col = inserter
            .column_mut(1)
            .as_slice::<i32>()
            .expect("Failed to insert version for way");
        let way_version_refs: Vec<&i32> =Way::extract(ways, |way| &way.version);
        let way_version_slice: Vec<i32> = way_version_refs.iter().map(|&&version| version).collect();
        let way_version_borrowed_slice: &[i32] = &way_version_slice;
        col.copy_from_slice(way_version_borrowed_slice);

        // timestamp insertion
        let mut col = inserter
            .column_mut(2)
            .as_text_view()
            .expect("Failed to insert timestamp for way");
        let way_timestamp_refs: Vec<&String> = Way::extract(ways, |way| &way.timestamp);
        let way_timestamp_slices: Vec<&str> = way_timestamp_refs.iter().map(|&ts| ts.as_str()).collect();
        let way_timestamp_slice: &[&str] = &way_timestamp_slices;

        for (index, timestamp) in way_timestamp_slice.iter().enumerate() {
            col.set_cell(index, Some(timestamp.as_bytes()));
        }

        // changeset insertion
        let col = inserter
            .column_mut(3)
            .as_slice::<i64>()
            .expect("Failed to insert changeset for way");
        let way_changeset_refs: Vec<&i64> = Way::extract(ways, |way| &way.changeset);
        let way_changeset_slice: Vec<i64> = way_changeset_refs.iter().map(|&&changeset| changeset).collect();
        let way_changeset_borrowed_slice: &[i64] = &way_changeset_slice;
        col.copy_from_slice(way_changeset_borrowed_slice);

        // uid insertion
        let col = inserter
            .column_mut(4)
            .as_slice::<i64>()
            .expect("Failed to insert uid for way");
        let way_uid_refs: Vec<&i64> = Way::extract(ways, |way| &way.uid);
        let way_uid_slice: Vec<i64> = way_uid_refs.iter().map(|&&uid| uid).collect();
        let way_uid_borrowed_slice: &[i64] = &way_uid_slice;
        col.copy_from_slice(way_uid_borrowed_slice);

        // user insertion
        let mut col = inserter
            .column_mut(5)
            .as_text_view()
            .expect("Failed to insert user for way");
        let way_user_refs: Vec<&String> = Way::extract(ways, |way| &way.user);
        let way_user_slices: Vec<&str> = way_user_refs.iter().map(|&ts| ts.as_str()).collect();
        let way_user_slice: &[&str] = &way_user_slices;

        for (index, user) in way_user_slice.iter().enumerate() {
            col.set_cell(index, Some(user.as_bytes()));
        }

        inserter.execute()?;
        Ok(())
    }

    /// Inserts a tag or tags associated with a node, way or relation into the database.
    ///
    /// # Arguments
    /// * `conn` - Connection to the sql database
    /// * `ids` - Identifier of the node, way or relation to which the tag is associated.
    /// * `keys` - Key of the tag/tags.
    /// * `values` - Value of the tag/tags.
    ///
    /// # Returns
    /// A result that, if Ok, signifies successful insertion, or if Err, contains an error.
    fn insert_tag(
        &self,
        conn: &Connection,
        ids: &[i64],
        keys: &[&str],
        values: &[&str],
        tag_type: TagType,
    ) -> Result<(), odbc_api::Error> {
        let table = tag_type.as_str();
        let sql = format!(
            "INSERT INTO [DenmarkMapsDB].[dbo].[{table}_tags] ({table}_id, [key], value) VALUES (?, ?, ?);",
        );
        // Connect to the database
        let prepared = conn.prepare(&sql)?;

        // Build buffer description
        let buffer_description = Tag::get_tag_buffer_descriptor();

        let mut inserter = prepared.into_column_inserter(ids.len(), buffer_description)?;
        inserter.set_num_rows(ids.len());

        ///// Fill the buffer with values column by column ////
        // id insertion
        let col = inserter
            .column_mut(0)
            .as_slice::<i64>()
            .expect("Failed to insert id for tag");
        col.copy_from_slice(ids);

        // key insertion
        let mut col = inserter
            .column_mut(1)
            .as_text_view()
            .expect("Failed to insert key for tag");

        for (index, key) in keys.iter().enumerate() {
            col.set_cell(index, Some(key.as_bytes()));
        }

        // value insertion
        let mut col = inserter
            .column_mut(2)
            .as_text_view()
            .expect("Failed to insert value for tag");

        for (index, value) in values.iter().enumerate() {
            col.set_cell(index, Some(value.as_bytes()));
        }

        inserter.execute()?;
        Ok(())
    }

    /// Inserts a way_node or way_nodes associated with a way or ways into the database.
    ///
    /// # Arguments
    /// * `conn` - Connection to the sql database
    /// * `ids` - Identifier of the node/way to which the tag is associated.
    /// * `keys` - Key of the tag/tags.
    /// * `values` - Value of the tag/tags.
    ///
    /// # Returns
    /// A result that, if Ok, signifies successful insertion, or if Err, contains an error.
    fn insert_way_node(
        &self,
        conn: &Connection,
        way_nodes: &[WayNode],
    ) -> Result<(), odbc_api::Error> {
        let sql = format!(
            "INSERT INTO [DenmarkMapsDB].[dbo].[way_nodes] (way_id, ref_id) VALUES (?, ?);",
        );
        // Connect to the database
        let prepared = conn.prepare(&sql)?;

        // Build buffer description
        let buffer_description = WayNode::get_way_node_buffer_descriptor();

        let mut inserter = prepared.into_column_inserter(way_nodes.len(), buffer_description)?;
        inserter.set_num_rows(way_nodes.len());

        ///// Fill the buffer with values column by column ////
        // way_id insertion
        let col = inserter
            .column_mut(0)
            .as_slice::<i64>()
            .expect("Failed to insert way_id for way_nodes");
        let way_ids: Vec<i64> = way_nodes.iter().map(|way_node| way_node.way_id).collect();
        let way_ids_borrowed_slice: &[i64] = &way_ids;
        col.copy_from_slice(way_ids_borrowed_slice);

        // ref_id insertion
        let col = inserter
            .column_mut(1)
            .as_slice::<i64>()
            .expect("Failed to insert ref_id for way_nodes");
        let ref_ids: Vec<i64> = way_nodes.iter().map(|way_node| way_node.ref_id).collect();
        let ref_ids_borrowed_slice: &[i64] = &ref_ids;
        col.copy_from_slice(ref_ids_borrowed_slice);

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
        let sql = "
        SELECT n.id, n.lat, n.lon, n.version, n.timestamp, n.changeset, n.uid, n.[user], t.[key], t.value
        FROM [DenmarkMapsDB].[dbo].[node] AS n
        LEFT JOIN [DenmarkMapsDB].[dbo].[node_tags] AS t ON n.id = t.node_id
        WHERE n.version >= ?
        ORDER BY n.id, t.[key];
        ";

        let conn = self.get_connection()?;
        let cursor = conn.execute(sql, (&min_version,))?;

        let mut nodes = Vec::new();
        let mut current_node_id = None;
        let mut current_node: Option<Node> = None;

        if let Some(mut cursor) = cursor {
            // Use schema in cursor to initialize a text buffer large enough to hold the largest
            // possible strings for each column up to an upper limit of 4KiB.
            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
            // Bind the buffer to the cursor. It is now being filled with every call to fetch.
            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            // Iterate over batches
            while let Some(batch) = row_set_cursor.fetch()? {
                // Within a batch, iterate over every row
                for row_index in 0..batch.num_rows() {
                    let node_id = parse_i64(batch.at(0, row_index))?;

                    if current_node_id != Some(node_id) {
                        if let Some(node) = current_node.take() {
                            nodes.push(node);
                        }

                        current_node = Some(Node {
                            id: node_id,
                            lat: parse_f64(batch.at(1, row_index))?,
                            lon: parse_f64(batch.at(2, row_index))?,
                            version: parse_i32(batch.at(3, row_index))?,
                            timestamp: parse_string(batch.at(4, row_index))?,
                            changeset: parse_i64(batch.at(5, row_index))?,
                            uid: parse_i64(batch.at(6, row_index))?,
                            user: parse_string(batch.at(7, row_index))?,
                            tags: Vec::new(),
                        });

                        current_node_id = Some(node_id);
                    }
                    let key = parse_string(batch.at(8, row_index))?;
                    let value = parse_string(batch.at(9, row_index))?;

                    if let Some(ref mut node) = current_node {
                        node.tags.push(Tag { key, value });
                    }
                }
                if let Some(ref node) = current_node {
                    nodes.push(node.clone());
                }
            }
        }

        Ok(nodes)
    }
}
