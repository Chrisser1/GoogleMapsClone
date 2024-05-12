use odbc_api::buffers::BufferDesc;

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    /// Provides descriptions of the internal buffer structures for ODBC connections, describing
    /// each field of the Tag struct in terms of database interaction.
    ///
    /// # Returns
    /// An array of BufferDesc elements, each describing the memory layout and properties
    /// of a field in the Tag struct for database operations.
    pub fn get_tag_buffer_descriptor() -> [BufferDesc; 3] {
        [
            BufferDesc::I64 { nullable: false },    // node id
            BufferDesc::Text { max_str_len: 128 },  // key
            BufferDesc::Text { max_str_len: 128 },  // value
        ]
    }
}

#[derive(Debug, Clone)]
pub enum TagType {
    Node,
    Way,
    Relation,
    Other(&'static str),  // Use &'static str to allow literal string references
}

// Example of usage
impl TagType {
    pub fn as_str(&self) -> &str {
        match self {
            TagType::Node => "node",
            TagType::Way => "way",
            TagType::Relation => "relation",
            TagType::Other(s) => s,
        }
    }
}
