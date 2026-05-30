use super::custom::QuerySort;
use super::models::ListStatusEnum;

#[derive(Debug, Clone)]
pub struct UserAnimeConfig {
    pub status: Option<ListStatusEnum>,
    pub sort:   QuerySort,
    pub limit:  u32,
    pub offset: u32,
}

impl Default for UserAnimeConfig {
    fn default() -> Self {
        Self {
            status: None,
            sort:   QuerySort::AnimeTitle,
            limit:  50,
            offset: 0,
        }
    }
}
