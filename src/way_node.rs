use odbc_api::buffers::BufferDesc;

#[derive(Debug, Clone)]
pub struct WayNode {
    pub way_id: i64,
    pub ref_id: i64,
}


impl WayNode {
    /// Provides descriptions of the internal buffer structures for ODBC connections, describing
    /// each field of the WayNode struct in terms of database interaction.
    ///
    /// # Returns
    /// An array of BufferDesc elements, each describing the memory layout and properties
    /// of a field in the WayNode struct for database operations.
    pub fn get_way_node_buffer_descriptor() -> [BufferDesc; 2] {
        [
            BufferDesc::I64 { nullable: false },    // way id
            BufferDesc::I64 { nullable: false },    // node id
        ]
    }
}
