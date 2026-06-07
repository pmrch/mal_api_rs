mod config;
mod search;
mod update;
mod user;

use reqwest::Response;
use update::UpdateBuilder;
use url::Url;

use super::helpers;
use super::shared::filter::SearchFilter;
use super::shared::traits::HasNode;
use super::shared::{api as models, models as custom};
use crate::prelude::sync::Arc;
use crate::prelude::{Client, Error, HashMap, HashSet, Result};

mod endpoints {
    pub const ANIME_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime";
    pub const RANKING_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime/ranking";
    pub const SEASONAL_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime/season";
    pub const SUGGESTION_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime/suggestions";
}

pub use search::AnimeSearchBuilder;
pub use user::UserAnimeBuilder;
