use serde::Deserialize;

use super::shared_models::{AlternativeTitles, Genre, MainPicture, Nsfw, Paging};
use super::{CompactString, OrderedFloat};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaNode {
    pub id:                 usize,
    pub title:              CompactString,
    pub main_picture:       Option<MainPicture>,
    pub alternative_titles: Option<AlternativeTitles>,
    pub start_date:         Option<chrono::NaiveDate>,
    pub end_date:           Option<chrono::NaiveDate>,
    pub synopsis:           Option<CompactString>,
    pub mean:               Option<OrderedFloat<f32>>,
    pub rank:               Option<usize>,
    pub popularity:         Option<usize>,
    pub num_list_users:     u32,
    pub num_scoring_users:  u32,
    pub nsfw:               Option<Nsfw>,
    pub genres:             Vec<Genre>,
    pub created_at:         chrono::DateTime<chrono::Utc>,
    pub updated_at:         chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaQuery {
    pub data:   Vec<MangaNode>,
    pub paging: Paging,
}

pub enum MangaType {
    
}