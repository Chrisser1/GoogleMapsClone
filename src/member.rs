use odbc_api::buffers::BufferDesc;
use sha2::{Sha256, Digest};

use crate::utils::MapsType;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: i64,
    pub ref_id: i64,
    pub maps_type: MapsType,
    pub role: String
}

impl Member {
    /// Creates a new member with a unique id using sha2 hasher
    pub fn new(ref_id: i64, maps_type: MapsType, role: String) -> Self {
        // Create a unique id based on ref_id and maps_type
        let mut hasher = Sha256::new();
        hasher.update(ref_id.to_be_bytes());
        hasher.update(maps_type.as_str().as_bytes());
        let result = hasher.finalize();
        let id = i64::from_be_bytes(result[0..8].try_into().unwrap_or([0; 8])); // Take the first 8 bytes for the i64 id

        Member {
            id,
            ref_id,
            maps_type,
            role,
        }
    }

    /// Provides descriptions of the internal buffer structures for ODBC connections, describing
    /// each field of the member struct in terms of database interaction.
    ///
    /// # Returns
    /// An array of BufferDesc elements, each describing the memory layout and properties
    /// of a field in the Member struct for database operations.
    pub fn get_member_buffer_descriptor() -> [BufferDesc; 7] {
        [
            BufferDesc::I64 { nullable: false },   // id
            BufferDesc::I64 { nullable: false },   // relation_id
            BufferDesc::I64 { nullable: false },   // node_id
            BufferDesc::I64 { nullable: false },   // way_id
            BufferDesc::I64 { nullable: false },   // relation_ref_id
            BufferDesc::Text { max_str_len: 32 },  // member_type
            BufferDesc::Text { max_str_len: 128 }, // role
        ]
    }

    /// Extracts references from a slice of members based on a provided extractor function.
    ///
    /// # Arguments
    /// * `members` - A slice of Member structs.
    /// * `extractor` - A function that takes a reference to a Member and returns a reference to a field.
    ///
    /// # Returns
    /// A vector of references as determined by the extractor function.
    pub fn extract<'a, T>(members: &'a [Self], extractor: fn(&'a Member) -> &'a T) -> Vec<&'a T> {
        members.iter().map(extractor).collect()
    }

    /// Gets the optional id based on the maps_type.
    ///
    /// # Arguments
    /// * `target_type` - The target MapsType to compare against.
    ///
    /// # Returns
    /// The id if the maps_type matches, otherwise -1.
    pub fn get_optional_id(&self, target_type: MapsType) -> &i64 {
        if self.maps_type == target_type {
            &self.ref_id
        } else {
            &-1
        }
    }
}
