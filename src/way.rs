use crate::tag::Tag;

#[derive(Debug, Clone)]
struct NodeRef {
    ref_id: i64, // Reference to a node ID
}

#[derive(Debug, Clone)]
pub struct Way {
    id: i64,
    version: u32,
    timestamp: String,
    changeset: i64,
    uid: i64,
    user: String,
    nodes: Vec<NodeRef>,  // List of node references
    tags: Vec<Tag>,       // List of tags
}

impl Way {
    pub fn new(id: i64, version: u32, timestamp: String, changeset: i64, uid: i64, user: String, nodes: Vec<NodeRef>, tags: Vec<Tag>) -> Self {
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
}
