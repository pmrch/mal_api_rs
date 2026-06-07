mod anime;
mod api;
mod helpers;
mod shared;

#[cfg(feature = "user")]
mod user;

#[cfg(feature = "user")]
use user::get_user_info;

use crate::prelude::{Error, Result, sync};

pub mod requests {
    pub use reqwest::header::{HeaderMap, HeaderValue};
    pub use reqwest::redirect::Policy;
    pub use reqwest::{Client, ClientBuilder};
}

pub use anime::{AnimeSearchBuilder, UserAnimeBuilder};
pub use api::MalApi;
pub use shared::filter::SearchFilter;

pub mod models {
    pub use super::shared::api::{AlternativeTitles, Anime, AnimeNode, ListStatusEnum, SeasonEnum, UserAnimeListEdge};
    pub use super::shared::models::{NumEps, QuerySort, SearchConfig, SearchMode, SortOrder};

    #[rustfmt::skip]
    #[cfg(feature = "user")]
    pub use super::user::{Statistics, UserInfo};
}
