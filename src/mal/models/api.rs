use ordered_float::OrderedFloat;
use serde::Deserialize;

use crate::mal::helpers::deserialize_date;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Anime {
    pub node: AnimeNode,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct AlternativeTitles {
    pub en:       String,
    pub ja:       String,
    pub synonyms: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Broadcast {
    pub day_of_the_week: String,
    pub start_time:      String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Genre {
    pub id:   u32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Season {
    pub season: SeasonEnum,
    pub year:   u16,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct AnimeStudio {
    pub id:   u32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct ListStatus {
    pub status:               ListStatusEnum,
    pub score:                u32,
    pub num_episodes_watched: u32,
    pub is_rewatching:        bool,
    pub start_date:           Option<chrono::NaiveDate>,
    pub finish_date:          Option<chrono::NaiveDate>,
    pub priority:             u32,
    pub num_times_rewatched:  u32,
    pub rewatch_value:        u32,
    pub tags:                 Vec<String>,
    pub comments:             Option<String>,
    pub updated_at:           chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct UserAnimeListEdge {
    pub node:        AnimeNode,
    pub list_status: Option<ListStatus>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct UserAnimeListQuery {
    pub data:   Vec<UserAnimeListEdge>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct AnimeNode {
    /// The identifier of this media on `MyAnimeList`.
    pub id:                 u32,
    /// The canonical title of the anime.
    pub title:              String,
    ///  The poster artwork of the anime.
    pub main_picture:       Option<MainPicture>,
    /// The alternative title of the anime.
    pub alternative_titles: Option<AlternativeTitles>,
    /// The average duration (in seconds) of the episodes.
    #[serde(rename = "average_episode_duration")]
    pub avg_ep_len:         Option<u32>,
    /// The day of week and time when the anime aired each week.
    pub broadcast:          Option<Broadcast>,
    /// The date of creation of entry on `MyAnimeList`.
    pub created_at:         Option<chrono::DateTime<chrono::Utc>>,
    /// The date at which the anime ended.
    #[serde(default, deserialize_with = "deserialize_date")]
    pub end_date:           Option<chrono::NaiveDate>,
    /// Genres of the anime.
    pub genres:             Option<Vec<Genre>>,
    /// The average star rating
    pub mean:               Option<OrderedFloat<f64>>,
    /// The type of this media (tv, ova, movie, special, ona, music, unknown).
    pub media_type:         Option<String>,
    /// The NSFW state for this media (white, gray, black).
    pub nsfw:               Option<String>,
    /// The number of episodes in this anime.
    pub num_episodes:       Option<u32>,
    /// The number of users that added this media to their favorites.
    pub num_favorites:      Option<usize>,
    /// The number of uses that added this media to their lists.
    pub num_list_users:     Option<usize>,
    /// The number of users that voted for the scores.
    pub num_scoring_users:  Option<usize>,
    /// The popularity rankings of this anime.
    pub popularity:         Option<u32>,
    /// The rankings of this anime.
    pub rank:               Option<u32>,
    /// The date at which the anime started.
    #[serde(default, deserialize_with = "deserialize_date")]
    pub start_date:         Option<chrono::NaiveDate>,
    /// The season at which the anime started broadcasting.
    pub start_season:       Option<Season>,
    /// An enumeration representing the broadcasting status of the anime
    pub status:             Option<Status>,
    /// The synopsis of the anime.
    pub synopsis:           Option<String>,
    /// The original work that inspired this anime
    pub source:             Option<String>,
    /// The studio that handled the animation
    pub studio:             Option<AnimeStudio>,
    /// The last time that the information is updated on `MyAnimeList`.
    pub updated_at:         Option<chrono::DateTime<chrono::Utc>>,
    /// Stats of the anime on the authorized user's list
    pub my_list_status:     Option<ListStatus>,
    /// Background story of the anime
    pub background:         Option<String>,
    /// A list of anime related to this anime
    pub related_anime:      Option<Vec<Anime>>,
    /// The rating of this anime (g All Ages, pg Children, pg-13 Teens 13 and
    /// Older, r 17+ (violence & profanity), r+ Profanity & Mild Nudity, rx
    /// Hentai).
    pub rating:             Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct AnimeQuery {
    pub data:   Vec<Anime>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Ranking {
    pub rank:          u32,
    pub previous_rank: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct RankingQueryData {
    pub node:    AnimeNode,
    pub ranking: Ranking,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct RankingQuery {
    pub data:   Vec<RankingQueryData>,
    pub paging: Paging,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct MainPicture {
    pub large:  String,
    pub medium: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Paging {
    pub next:     Option<String>,
    pub previous: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SeasonEnum {
    Winter,
    Spring,
    Summer,
    Fall,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ListStatusEnum {
    Watching,
    Completed,
    OnHold,
    Dropped,
    PlanToWatch,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    FinishedAiring,
    CurrentlyAiring,
    NotYetAired,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum AnimeRankingType {
    All,
    Airing,
    Upcoming,
    Tv,
    Ova,
    Movie,
    Special,
    ByPopularity,
    Favorite,
    #[serde(other)]
    Unknown,
}
