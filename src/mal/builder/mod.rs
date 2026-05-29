mod anime;
mod filter;
mod impls;
mod traits;

pub(super) use traits::HasNode;

use super::helpers::matches_title;
use super::requests;
use crate::prelude::sync::Arc;
use crate::prelude::{Error, HashMap, HashSet, Result, my_hash_map};

pub(super) mod endpoints {
    pub const ANIME_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime";
    pub const RANKING_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime/ranking";
    pub const SEASONAL_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime/season";
    pub const SUGGESTION_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime/suggestions";
}

mod models {
    pub(super) use crate::mal::models::{EpLengthMins, NumEps, SeasonEnum};
    pub(super) use crate::mal::{SearchConfig, SearchFilter, SearchMode, SortOrder};
}

mod api {
    pub(super) use crate::models::{Anime, AnimeNode, AnimeQuery, AnimeRankingType, RankingQuery, RankingQueryData};
}

pub use anime::AnimeSearchBuilder;
pub use filter::SearchFilter;
