mod api;
mod auth;
mod builder;
mod helpers;
mod impls;
pub mod models;

use crate::prelude::{Result, models as gmodels, sync};

pub mod requests {
    pub use reqwest::header::{HeaderMap, HeaderValue};
    pub use reqwest::redirect::Policy;
    pub use reqwest::{Client, ClientBuilder, Response};
    pub use url::Url;
}

pub use api::MalApi;
pub use auth::UserAnimeBuilder;
pub use builder::{AnimeSearchBuilder, SearchFilter};
pub use models::{NumEps, SearchConfig, SearchMode, SortOrder};
