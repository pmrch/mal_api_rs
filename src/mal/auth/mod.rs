mod anime;
mod auth_models;
mod impls;
mod update;

pub use anime::UserAnimeBuilder;
pub use auth_models::QuerySort;
use update::UpdateBuilder;

use super::builder::endpoints::ANIME_ENDPOINT;
use super::builder::{HasNode, SearchFilter};
use super::requests;
use crate::my_hash_map;
use crate::prelude::sync::Arc;
use crate::prelude::{Error, HashMap, Result};

mod models {
    pub use crate::mal::models::{ListStatus, ListStatusEnum, SearchMode, UserAnimeListEdge, UserAnimeListQuery};
}
