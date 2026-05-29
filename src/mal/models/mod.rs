mod abstractions;
mod api;

const DEFAULT_FIELDS: &[&str] = &["start_date", "alternative_titles"];
const DEFAULT_THRESHOLD: f64 = 0.75f64;
const DEFAULT_NUM_TITLES: usize = 15;
const DEFAULT_LIMIT: u32 = 50;
const DEFAULT_OFFSET: u32 = 0;

pub use abstractions::{EpLengthMins, NumEps, SearchConfig, SearchMode, SortOrder};
pub use api::{
    AlternativeTitles, Anime, AnimeNode, AnimeQuery, AnimeRankingType, Broadcast, Genre, ListStatus, ListStatusEnum, MainPicture,
    RankingQuery, RankingQueryData, SeasonEnum, Status, UserAnimeListEdge, UserAnimeListQuery,
};
