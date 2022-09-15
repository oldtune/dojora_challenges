use super::schema::challenge;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[diesel(table_name = challenge)]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
}
