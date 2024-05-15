use odbc_api::buffers::BufferDesc;

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

impl Tag {
    pub fn new(key: String, value: String) -> Self {
        Tag {
            key,
            value,
        }
    }

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
