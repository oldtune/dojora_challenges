pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub created_at: i64,
}

impl Challenge {
    pub fn new(id: uuid::Uuid, title: String, description: String, created_at: i64) -> Self {
        Self {
            id,
            title,
            description,
            created_at,
        }
    }
}
