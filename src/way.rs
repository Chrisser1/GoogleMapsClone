use crate::{tag::Tag, way_node::WayNode};
use odbc_api::buffers::BufferDesc;

/// Represents a pairing of a way ID with a tag.
#[derive(Debug, Clone)]
pub struct WayTag {
    pub way_id: i64,
    pub tag: Tag,
}

impl WayTag {
    /// Collects node IDs, tag keys, and tag values from a slice of WayTag structs.
    ///
    /// # Arguments
    /// * `node_tags` - A slice of WayTag structs to collect data from.
    ///
    /// # Returns
    /// A tuple of three vectors containing node IDs, tag keys, and tag values respectively.
    pub fn collect_tag_data(node_tags: &[WayTag]) -> (Vec<i64>, Vec<&str>, Vec<&str>) {
        let mut ids = Vec::new();
        let mut keys = Vec::new();
        let mut values = Vec::new();

        for node_tag in node_tags {
            ids.push(node_tag.way_id);
            keys.push(node_tag.tag.key.as_str());
            values.push(node_tag.tag.value.as_str());
        }

        (ids, keys, values)
    }
}


#[derive(Debug, Clone)]
pub struct Way {
    pub id: i64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
    pub nodes: Vec<WayNode>,  // List of node references
    pub tags: Vec<Tag>,       // List of tags
}

impl Way {
    pub fn new(id: i64, version: i32, timestamp: String, changeset: i64, uid: i64, user: String, nodes: Vec<WayNode>, tags: Vec<Tag>) -> Self {
        Way {
            id,
            version,
            timestamp,
            changeset,
            uid,
            user,
            nodes,
            tags,
        }
    }

    /// Extracts references from a slice of ways based on a provided extractor function.
    ///
    /// # Arguments
    /// * `ways` - A slice of Way structs.
    /// * `extractor` - A function that takes a reference to a Way and returns a reference to a field.
    ///
    /// # Returns
    /// A vector of references as determined by the extractor function.
    pub fn extract<'a, T>(ways: &'a [Self], extractor: fn(&'a Way) -> &'a T) -> Vec<&'a T> {
        ways.iter().map(extractor).collect()
    }

    /// Provides descriptions of the internal buffer structures for ODBC connections, describing
    /// each field of the Way struct in terms of database interaction.
    ///
    /// # Returns
    /// An array of BufferDesc elements, each describing the memory layout and properties
    /// of a field in the Way struct for database operations.
    pub fn get_way_buffer_descriptor() -> [BufferDesc; 6] {
        [
            BufferDesc::I64 { nullable: false },   // id
            BufferDesc::I32 { nullable: false },   // version
            BufferDesc::Text { max_str_len: 32 },  // timestamp
            BufferDesc::I64 { nullable: false },   // changeset
            BufferDesc::I64 { nullable: false },   // uid
            BufferDesc::Text { max_str_len: 128 }, // user
        ]
    }

    /// Extracts way ID and tag pairs from a slice of ways.
    ///
    /// # Arguments
    /// * `ways` - A slice of way structs from which way IDs and tags are extracted.
    ///
    /// # Returns
    /// A vector of WayTag structs, each containing a way ID and a corresponding tag.
    pub fn extract_way_tags<'a>(ways: &'a [Self]) -> Vec<WayTag> {
        ways.iter()
            .flat_map(|way| way.tags.iter().map(move |tag| WayTag {
                way_id: way.id,
                tag: tag.clone(),
            }))
            .collect()
    }

    /// Extracts way_nodes from a slice of ways.
    ///
    /// # Arguments
    /// * `ways` - A slice of way structs.
    ///
    /// # Returns
    /// A vector of WayNode structs, each containing a way ID and ref ID.
    pub fn extract_way_nodes<'a>(ways: &'a [Self]) -> Vec<WayNode> {
        ways.iter()
            .flat_map(|way| way.nodes.iter().map(move |way_node| WayNode {
                way_id: way.id,
                ref_id: way_node.ref_id,
            }))
            .collect()
    }
}
