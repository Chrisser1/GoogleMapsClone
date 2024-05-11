use odbc_api::buffers::BufferDesc;
use crate::tag::Tag;

/// Represents a pairing of a node ID with a tag.
#[derive(Debug, Clone)]
pub struct NodeTag {
    pub node_id: i64,
    pub tag: Tag,
}

impl NodeTag {
    /// Collects node IDs, tag keys, and tag values from a slice of NodeTag structs.
    ///
    /// # Arguments
    /// * `node_tags` - A slice of NodeTag structs to collect data from.
    ///
    /// # Returns
    /// A tuple of three vectors containing node IDs, tag keys, and tag values respectively.
    pub fn collect_tag_data(node_tags: &[NodeTag]) -> (Vec<i64>, Vec<&str>, Vec<&str>) {
        let mut ids = Vec::new();
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for node_tag in node_tags {
            ids.push(node_tag.node_id);
            keys.push(node_tag.tag.key.as_str());
            values.push(node_tag.tag.value.as_str());
        }

        (ids, keys, values)
    }
}


/// Represents a geographic node with various properties and metadata.
///
/// # Fields
/// * `id` - A unique identifier for the node as an i64.
/// * `lat` - The latitude of the node as a f64.
/// * `lon` - The longitude of the node as a f64.
/// * `version` - The version of the node, represented as an i32.
/// * `timestamp` - A String indicating the time at which the node was last updated.
/// * `changeset` - An i64 identifier for a group of changes made together.
/// * `uid` - The user ID as an i64 of the user who last modified the node.
/// * `user` - A String representing the username of the last modifier.
/// * `tags` - A Vec<Tag> for additional metadata about the node.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
    pub tags: Vec<Tag>,
}

impl Node {
    /// Extracts references from a slice of nodes based on a provided extractor function.
    ///
    /// # Arguments
    /// * `nodes` - A slice of Node structs.
    /// * `extractor` - A function that takes a reference to a Node and returns a reference to a field.
    ///
    /// # Returns
    /// A vector of references as determined by the extractor function.
    pub fn extract<'a, T>(nodes: &'a [Self], extractor: fn(&'a Node) -> &'a T) -> Vec<&'a T> {
        nodes.iter().map(extractor).collect()
    }

    /// Provides descriptions of the internal buffer structures for ODBC connections, describing
    /// each field of the Node struct in terms of database interaction.
    ///
    /// # Returns
    /// An array of BufferDesc elements, each describing the memory layout and properties
    /// of a field in the Node struct for database operations.
    pub fn get_node_buffer_descriptor() -> [BufferDesc; 8] {
        [
            BufferDesc::I64 { nullable: false },   // id
            BufferDesc::F64 { nullable: false },   // lat
            BufferDesc::F64 { nullable: false },   // lon
            BufferDesc::I32 { nullable: false },   // version
            BufferDesc::Text { max_str_len: 32 },  // timestamp
            BufferDesc::I64 { nullable: false },   // changeset
            BufferDesc::I64 { nullable: false },   // uid
            BufferDesc::Text { max_str_len: 128 }, // user
        ]
    }

    /// Extracts node ID and tag pairs from a slice of nodes.
    ///
    /// # Arguments
    /// * `nodes` - A slice of Node structs from which node IDs and tags are extracted.
    ///
    /// # Returns
    /// A vector of NodeTag structs, each containing a node ID and a corresponding tag.
    pub fn extract_node_tags<'a>(nodes: &'a [Self]) -> Vec<NodeTag> {
        nodes.iter()
            .flat_map(|node| node.tags.iter().map(move |tag| NodeTag {
                node_id: node.id,
                tag: tag.clone(),
            }))
            .collect()
    }
}
