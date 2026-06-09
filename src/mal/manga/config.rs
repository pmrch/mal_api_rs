use super::models::{MangaListStatusEnum, MangaQuerySort};

#[derive(Debug, Default)]
pub struct UserMangeConfig {
    pub status: MangaListStatusEnum,
    pub sort:   MangaQuerySort,
    pub limit:  u32,
    pub offset: u32,
}
