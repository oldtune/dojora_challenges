use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    pub page_index: PageIndex,
    pub page_size: PageSize,
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct PageIndex(u8);

impl PageIndex {
    pub fn inner(&self) -> u8 {
        self.0
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct PageSize(u8);

impl PageSize {
    pub fn inner(&self) -> u8 {
        self.0
    }
}
