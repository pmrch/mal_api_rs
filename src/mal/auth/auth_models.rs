#[derive(Debug, Clone)]
pub struct UserAnimeConfig {
    pub status: Option<super::models::ListStatusEnum>,
    pub sort:   QuerySort,
    pub limit:  u32,
    pub offset: u32,
}

#[derive(Debug, Clone, Default)]
pub enum QuerySort {
    ListScore,
    ListUpdatedAt,
    #[default]
    AnimeTitle,
    AnimeStartDate,
    AnimeId,
}
