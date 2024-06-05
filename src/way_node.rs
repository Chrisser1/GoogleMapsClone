#[derive(Debug, Clone)]
pub struct WayNode {
    pub way_id: i64,
    pub ref_id: i64,
}


impl WayNode {
    pub fn new(way_id: i64, ref_id: i64) -> Self {
        WayNode {
            way_id,
            ref_id,
        }
    }
}
