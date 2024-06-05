#[derive(Debug, Clone, Default, sqlx::FromRow)]
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
}
