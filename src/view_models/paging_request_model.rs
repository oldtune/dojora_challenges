use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PagingModel {
    pub page_index: i64,
    pub page_size: i64,
    pub keyword: String,
}