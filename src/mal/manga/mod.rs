mod config;
mod filter;
mod impls;
mod models;
mod search;
mod update;
mod user;

use std::collections::{HashMap, HashSet};

use compact_str::{CompactString, format_compact};
use ordered_float::OrderedFloat;
use url::Url;

use super::shared::traits::MangaHasNode;
use super::{helpers, requests};
use crate::my_hash_map;
use crate::prelude::sync::Arc;
use crate::prelude::{Error, Result};

mod shared_models {
    pub use crate::mal::shared::api::{AlternativeTitles, AnimeNode, Genre, MainPicture, Nsfw, Paging, Ranking};
    pub use crate::mal::shared::models::{SearchConfig, SearchMode, SortOrder};
}

mod endpoints {
    pub const MANGA_ENDPOINT: super::CompactString = super::CompactString::const_new("https://api.myanimelist.net/v2/manga");
}

pub mod api {
    pub use super::models::{Manga, MangaNode, MangaRankingQueryData, MangaRankingType, MangaType};
}

pub mod manga_models {
    pub use super::models::{MangaListStatusEnum, MangaSearchFilter, MangaStatus};
    pub use super::search::MangaSearchBuilder;
}
