use crate::{tag::Tag, utils::MapsTag, way_node::WayNode};

#[derive(Debug, Clone)]
pub struct Way {
    pub id: i64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
    pub nodes: Vec<WayNode>,
    pub tags: Vec<Tag>,
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

    /// Extracts way ID and tag pairs from a slice of ways.
    ///
    /// # Arguments
    /// * `ways` - A slice of way structs from which way IDs and tags are extracted.
    ///
    /// # Returns
    /// A vector of MapsTag structs, each containing a way ID and a corresponding tag.
    pub fn extract_way_tags<'a>(ways: &'a [Self]) -> Vec<MapsTag> {
        ways.iter()
            .flat_map(|way| way.tags.iter().map(move |tag| MapsTag {
                id: way.id,
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
