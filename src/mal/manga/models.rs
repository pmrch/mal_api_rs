use serde::{Deserialize, Serialize};

pub use super::filter::MangaSearchFilter;
use super::shared_models::{AlternativeTitles, AnimeNode, Genre, MainPicture, Nsfw, Paging, Ranking};
use super::{CompactString, OrderedFloat};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaQuery {
    pub data:   Vec<Manga>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Manga {
    pub node: MangaNode,
}

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
    pub num_list_users:     Option<u32>,
    pub num_scoring_users:  Option<u32>,
    pub nsfw:               Option<Nsfw>,
    pub genres:             Option<Vec<Genre>>,
    pub created_at:         Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at:         Option<chrono::DateTime<chrono::Utc>>,
    pub media_type:         Option<MangaType>,
    pub status:             Option<MangaStatus>,
    pub my_list_status:     Option<MangaListStatus>,
    pub num_volumes:        Option<u32>,
    pub num_chapters:       Option<u32>,
    pub authors:            Option<Vec<MangaAuthor>>,
    pub ranking:            Option<Ranking>,
    pub pictures:           Option<MangaPictures>,
    pub background:         Option<CompactString>,
    pub related_anime:      Option<Vec<MangaRelatedAnime>>,
    pub related_manga:      Option<Vec<MangaRelationManga>>,
    pub recommendations:    Option<Vec<MangaRecommendation>>,
    pub serialization:      Option<Vec<Serialization>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct UserMangaListEdge {
    pub node:        MangaNode,
    pub list_status: MangaListStatus,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct UserMangaListQuery {
    pub data:   Vec<UserMangaListEdge>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct SerializationNode {
    pub id:   usize,
    pub name: CompactString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Serialization {
    pub node: SerializationNode,
    pub role: CompactString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaRecommendation {
    pub node:                MangaNode,
    pub num_recommendations: usize,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaRelationManga {
    pub node:                    MangaNode,
    pub relation_type:           RelationType,
    pub relation_type_formatted: CompactString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaRelatedAnime {
    pub node:                    AnimeNode,
    pub relation_type:           RelationType,
    pub relation_type_formatted: CompactString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaRankingQuery {
    pub data:   Vec<MangaRankingQueryData>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaRankingQueryData {
    pub node:    MangaNode,
    pub ranking: Ranking,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaPictures {
    pub large:  Option<CompactString>,
    pub medium: CompactString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaListStatus {
    pub status:            MangaListStatusEnum,
    pub score:             u32,
    pub num_volumes_read:  u32,
    pub num_chapters_read: u32,
    pub is_rereading:      bool,
    pub start_date:        Option<chrono::NaiveDate>,
    pub finish_date:       Option<chrono::NaiveDate>,
    pub priority:          u32,
    pub num_times_reread:  u32,
    pub reread_value:      u32,
    pub tags:              Vec<CompactString>,
    pub comments:          Option<CompactString>,
    pub updated_at:        chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaAuthor {
    pub node: MangaAuthorNode,
    pub role: CompactString,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MangaAuthorNode {
    pub id:         usize,
    pub first_name: Option<CompactString>,
    pub last_name:  Option<CompactString>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MangaRankingType {
    All,
    Manga,
    Novels,
    Oneshots,
    Doujin,
    Manhwa,
    Manhua,
    Bypopularity,
    Favorite,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RelationType {
    Sequel,
    Prequel,
    AlternativeSetting,
    AlternativeVersion,
    SideStory,
    ParentStory,
    Summary,
    FullStory,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MangaType {
    Oel,
    Manhua,
    Manhwa,
    Doujinshi,
    OneShot,
    Novel,
    Manga,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MangaStatus {
    Finished,
    CurrentlyPublishing,
    NoyYetPublished,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, Default)]
#[serde(rename_all = "snake_case")]
pub enum MangaListStatusEnum {
    #[default]
    Reading,
    Completed,
    OnHold,
    Dropped,
    PlanToRead,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MangaQuerySort {
    ListScore,
    ListUpdatedAt,
    #[default]
    MangaTitle,
    MangaStartDate,
    MangaId,
}
