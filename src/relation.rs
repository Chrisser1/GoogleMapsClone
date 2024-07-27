use sqlx::{FromRow, sqlite::SqliteRow, Row};
use crate::{member::Member, tag::Tag, utils::{MapsTag, MapsType}};

#[derive(Clone, Debug)]
pub struct Relation {
    pub id: i64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
    pub members: Vec<Member>,
    pub tags: Vec<Tag>,
}

impl Relation {
    pub fn new(id: i64, version: i32, timestamp: String, changeset: i64, uid: i64, user: String, members: Vec<Member>, tags: Vec<Tag>) -> Self {
        Relation {
            id,
            version,
            timestamp,
            changeset,
            uid,
            user,
            members,
            tags
        }
    }

    /// Extracts references from a slice of relations based on a provided extractor function.
    ///
    /// # Arguments
    /// * `relations` - A slice of Relation structs.
    /// * `extractor` - A function that takes a reference to a Relation and returns a reference to a field.
    ///
    /// # Returns
    /// A vector of references as determined by the extractor function.
    pub fn extract<'a, T>(relations: &'a [Self], extractor: fn(&'a Relation) -> &'a T) -> Vec<&'a T> {
        relations.iter().map(extractor).collect()
    }

    /// Extracts relation ID and tag pairs from a slice of relations.
    ///
    /// # Arguments
    /// * `relations` - A slice of relation structs from which relation IDs and tags are extracted.
    ///
    /// # Returns
    /// A vector of MapsTag structs, each containing a relation ID and a corresponding tag.
    pub fn extract_relation_tags<'a>(relations: &'a [Self]) -> Vec<MapsTag> {
        relations.iter()
            .flat_map(|relation| relation.tags.iter().map(move |tag| MapsTag {
                id: relation.id,
                tag: tag.clone(),
            }))
            .collect()
    }

    /// Extracts members from a slice of relations along with their relation IDs.
    ///
    /// # Arguments
    /// * `relations` - A slice of Relation structs.
    ///
    /// # Returns
    /// A vector of tuples where each tuple contains a relation ID and a Member struct.
    pub fn extract_members(relations: &[Relation]) -> Vec<(i64, Member)> {
        relations.iter()
            .flat_map(|relation| {
                relation.members.iter().cloned().map(move |member| (relation.id, member))
            })
            .collect()
    }
}

impl FromRow<'_, SqliteRow> for Relation {
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
                    let key = parts.next()?.to_string();
                    let value = parts.next()?.to_string();
                    Some(Tag { key, value })
                })
                .collect()
        } else {
            Vec::new()
        };

        let members_str: Option<String> = row.try_get("members").ok();
        let members = if let Some(members_str) = members_str {
            members_str.split(',')
                .filter_map(|member| {
                    let mut parts = member.splitn(6, ':');
                    let id = parts.next()?.parse::<i64>().ok()?;
                    let node_id = parts.next()?.parse::<i64>().ok();
                    let way_id = parts.next()?.parse::<i64>().ok();
                    let relation_ref_id = parts.next()?.parse::<i64>().ok();
                    let maps_type_str = parts.next()?;
                    let maps_type = maps_type_str.parse::<MapsType>().ok()?;
                    let role = parts.next()?.to_string();

                    let ref_id = match maps_type {
                        MapsType::Node => node_id,
                        MapsType::Way => way_id,
                        MapsType::Relation => relation_ref_id,
                        _ => None,
                    }?;

                    Some(Member { id, ref_id, maps_type, role })
                })
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self {
            id,
            version,
            timestamp,
            changeset,
            uid,
            user,
            members,
            tags,
        })
    }
}
