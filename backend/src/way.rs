use sqlx::{FromRow, sqlite::SqliteRow, Row};
use crate::tag::Tag;

#[derive(Debug, Clone)]
pub struct Way {
    pub id: i64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
    pub node_refs: Vec<i64>,
    pub tags: Vec<Tag>,
}

impl Way {
    pub fn new(id: i64, version: i32, timestamp: String, changeset: i64, uid: i64, user: String, node_ids: Vec<i64>, tags: Vec<Tag>) -> Self {
        Way {
            id,
            version,
            timestamp,
            changeset,
            uid,
            user,
            node_refs: node_ids,
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

    /// Extracts references from a slice of ways based on a provided extractor function.
    ///
    /// # Arguments
    /// * `ways` - A slice of Way structs.
    /// * `extractor` - A function that takes a reference to a Way and returns a reference to a field.
    ///
    /// # Returns
    /// A vector of references as determined by the extractor function.
    pub fn extract_node_ref<'a, T>(ways: &'a [Self], extractor: fn(&'a Way) -> &'a T) -> Vec<&'a T> {
        ways.iter().map(extractor).collect()
    }

    /// Extracts way ID and node_ref pairs from a slice of ways.
    ///
    /// # Arguments
    /// * `ways` - A slice of way structs from which way IDs and node_refs are extracted.
    ///
    /// # Returns
    /// A vector of tuples, each containing a way ID and a corresponding node_ref.
    pub fn extract_way_node_refs(ways: &[Self]) -> Vec<(i64, i64)> {
        ways.iter()
            .flat_map(|way| way.node_refs.iter().map(move |&node_ref| (way.id, node_ref)))
            .collect()
    }
}

impl FromRow<'_, SqliteRow> for Way {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id")?;
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

        let node_refs_str: Option<String> = row.try_get("node_refs").ok();
        let node_refs = if let Some(node_refs_str) = node_refs_str {
            node_refs_str.split(',')
                .filter_map(|node_ref| node_ref.parse::<i64>().ok())
                .collect()
        } else {
            Vec::new()
        };

        // Node references will be handled separately
        Ok(Self {
            id,
            version,
            timestamp,
            changeset,
            uid,
            user,
            node_refs: node_refs, // Will be populated later
            tags,
        })
    }
}
