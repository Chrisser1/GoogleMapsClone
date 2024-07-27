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
    pub fn new(relation_id: i64, ref_id: i64, maps_type: MapsType, role: String) -> Self {
        // Create a unique id based on relation_id, ref_id, maps_type, and role
        let mut hasher = Sha256::new();
        hasher.update(relation_id.to_be_bytes());
        hasher.update(ref_id.to_be_bytes());
        hasher.update(maps_type.as_str().as_bytes());
        hasher.update(role.as_str().as_bytes());
        let result = hasher.finalize();
        let id = i64::from_be_bytes(result[0..8].try_into().unwrap_or([0; 8])); // Take the first 8 bytes for the i64 id

        Member {
            id,
            ref_id,
            maps_type,
            role,
        }
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
}
