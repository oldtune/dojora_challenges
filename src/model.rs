use super::schema::challenge;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Insertable)]
#[table_name = "challenge"]
pub struct Challenge {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: String,
}
