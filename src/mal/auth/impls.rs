impl super::HasNode for super::models::UserAnimeListEdge {
    fn node(&self) -> &crate::models::AnimeNode { &self.node }
}

impl std::fmt::Display for super::auth_models::QuerySort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnimeId => write!(f, "anime_id"),
            Self::AnimeTitle => write!(f, "anime_title"),
            Self::AnimeStartDate => write!(f, "anime_start_date"),
            Self::ListScore => write!(f, "list_score"),
            Self::ListUpdatedAt => write!(f, "list_updated_at"),
        }
    }
}

impl Default for super::auth_models::UserAnimeConfig {
    fn default() -> Self {
        Self {
            status: None,
            sort:   super::auth_models::QuerySort::default(),
            limit:  100,
            offset: 0,
        }
    }
}
