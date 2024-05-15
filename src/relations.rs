use odbc_api::buffers::BufferDesc;

use crate::{member::Member, tag::Tag, utils::MapsTag};

#[derive(Clone, Debug)]
pub struct Relation {
    pub id: i64,
    pub version: i32,
    pub timestamp: String,
    pub changeset: i64,
    pub uid: i64,
    pub user: String,
    pub member: Vec<Member>,
    pub tags: Vec<Tag>,
}

impl Relation {
    /// Provides descriptions of the internal buffer structures for ODBC connections, describing
    /// each field of the relation struct in terms of database interaction.
    ///
    /// # Returns
    /// An array of BufferDesc elements, each describing the memory layout and properties
    /// of a field in the Relation struct for database operations.
    pub fn get_relation_buffer_descriptor() -> [BufferDesc; 6] {
        [
            BufferDesc::I64 { nullable: false },   // id
            BufferDesc::I32 { nullable: false },   // version
            BufferDesc::Text { max_str_len: 32 },  // timestamp
            BufferDesc::I64 { nullable: false },   // changeset
            BufferDesc::I64 { nullable: false },   // uid
            BufferDesc::Text { max_str_len: 128 }, // user
        ]
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

    /// Extracts members from a slice of relations.
    ///
    /// # Arguments
    /// * `relations` - A slice of Relation structs.
    ///
    /// # Returns
    /// A vector of Member structs, each containing a way ID and ref ID.
    pub fn extract_members(relations: &[Relation]) -> Vec<Member> {
        relations.iter()
            .flat_map(|relation| relation.member.iter().cloned())
            .collect()
    }
}
