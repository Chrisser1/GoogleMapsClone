pub struct Node {
    pub id: i64,
    pub lat: f64,
    pub lon: f64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
}

/// Extracts references to the IDs from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the i64 IDs of the nodes.
pub fn extract_node_ids<'a>(nodes: &'a [Node]) -> Vec<&'a i64> {
    nodes.iter().map(|node| &node.id).collect()
}

/// Extracts references to the changesets from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the i64 changesets of the nodes.
pub fn extract_node_changesets<'a>(nodes: &'a [Node]) -> Vec<&'a i64> {
    nodes.iter().map(|node| &node.changeset).collect()
}

/// Extracts references to the uids from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the i64 uids of the nodes.
pub fn extract_node_uids<'a>(nodes: &'a [Node]) -> Vec<&'a i64> {
    nodes.iter().map(|node| &node.uid).collect()
}

/// Extracts references to the lats from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the f64 lats of the nodes.
pub fn extract_node_lats<'a>(nodes: &'a [Node]) -> Vec<&'a f64> {
    nodes.iter().map(|node| &node.lat).collect()
}

/// Extracts references to the lons from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the f64 lons of the nodes.
pub fn extract_node_lons<'a>(nodes: &'a [Node]) -> Vec<&'a f64> {
    nodes.iter().map(|node| &node.lon).collect()
}

/// Extracts references to the versions from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the i32 versions of the nodes.
pub fn extract_node_versions<'a>(nodes: &'a [Node]) -> Vec<&'a i32> {
    nodes.iter().map(|node| &node.version).collect()
}

/// Extracts references to the timestamps from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the String timestamps of the nodes.
pub fn extract_node_timestamps<'a>(nodes: &'a [Node]) -> Vec<&'a String> {
    nodes.iter().map(|node| &node.timestamp).collect()
}

/// Extracts references to the users from a slice of nodes and returns them as a vector of references.
///
/// # Arguments
/// * `nodes` - A slice of Node structs.
///
/// # Returns
/// A vector of references to the String users of the nodes.
pub fn extract_node_users<'a>(nodes: &'a [Node]) -> Vec<&'a String> {
    nodes.iter().map(|node| &node.user).collect()
}
