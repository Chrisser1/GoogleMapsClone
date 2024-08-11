use sqlx::{FromRow, sqlite::SqliteRow, Row};

use crate::{
    osm_entities::Tag,
    utils::MapsTag
};

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
    pub fn new(id: i64, lat: f64, lon: f64, version: i32, timestamp: String, changeset: i64, uid: i64, user: String, tags: Vec<Tag>) -> Self {
        Node {
            id,
            lat,
            lon,
            version,
            timestamp,
            changeset,
            uid,
            user,
            tags,
        }
    }

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

    /// Extracts node ID and tag pairs from a slice of nodes.
    ///
    /// # Arguments
    /// * `nodes` - A slice of Node structs from which node IDs and tags are extracted.
    ///
    /// # Returns
    /// A vector of MapsTag structs, each containing a node ID and a corresponding tag.
    pub fn extract_node_tags<'a>(nodes: &'a [Self]) -> Vec<MapsTag> {
        nodes.iter()
            .flat_map(|node| node.tags.iter().map(move |tag| MapsTag {
                id: node.id,
                tag: tag.clone(),
            }))
            .collect()
    }
}

impl FromRow<'_, SqliteRow> for Node {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        let id: i64 = row.try_get("id")?;
        let lat: f64 = row.try_get("lat")?;
        let lon: f64 = row.try_get("lon")?;
        let version: i32 = row.try_get("version")?;
        let timestamp: String = row.try_get("timestamp")?;
        let changeset: i64 = row.try_get("changeset")?;
        let uid: i64 = row.try_get("uid")?;
        let user: String = row.try_get("user")?;

        let tags_str: Option<String> = row.try_get("tags").ok();
        let tags = if let Some(tags_str) = tags_str {
            tags_str.split(',')
                .filter_map(|tag| {
                    let mut parts = tag.splitn(2, ':');
                    let key = parts.next().unwrap_or_default().to_string();
                    let value = parts.next().unwrap_or_default().to_string();
                    if key.is_empty() || value.is_empty() {
                        None
                    } else {
                        Some(Tag { key, value })
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self {
            id,
            lat,
            lon,
            version,
            timestamp,
            changeset,
            uid,
            user,
            tags,
        })
    }
}
