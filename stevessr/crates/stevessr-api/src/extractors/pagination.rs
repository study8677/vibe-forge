use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 0 }
fn default_per_page() -> u32 { 30 }

impl Pagination {
    pub fn offset(&self) -> i64 {
        (self.page * self.per_page) as i64
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}
