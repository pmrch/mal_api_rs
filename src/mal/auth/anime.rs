use super::auth_models::{QuerySort, UserAnimeConfig};
use super::models::{ListStatus, ListStatusEnum, SearchMode, UserAnimeListEdge, UserAnimeListQuery};
use super::requests::{Client, Response, Url};
use super::{ANIME_ENDPOINT, Arc, Error, HashMap, Result, SearchFilter, UpdateBuilder, my_hash_map};

const USERS_ENDPOINT: &str = "https://api.myanimelist.net/v2/users";
const SELF_LIST: &str = "@me";

pub struct UserAnimeBuilder {
    client:       Arc<Client>,
    access_token: Option<Arc<str>>,
    filter:       Option<SearchFilter>,
    search_mode:  SearchMode,
    config:       UserAnimeConfig,
}

impl UserAnimeBuilder {
    pub fn new(client: Arc<Client>, access_token: Option<Arc<str>>) -> Self {
        Self {
            client,
            access_token,
            filter: None,
            search_mode: SearchMode::All,
            config: UserAnimeConfig::default(),
        }
    }

    /// Set the limit of anime per page
    pub const fn limit(mut self, limit: u32) -> Self {
        self.config.limit = limit;
        self
    }

    /// Controls how multiple filters are evaluated against each result.
    /// - [`SearchMode::All`] — all filters must match (AND logic)
    /// - [`SearchMode::Any`] — at least one filter must match (OR logic)
    /// - [`SearchMode::AtLeast(n)`] — at least `n` filters must match
    ///
    /// Optional, defaults to [`SearchMode::All`]
    pub const fn filter_mode(mut self, search_mode: SearchMode) -> Self {
        self.search_mode = search_mode;
        self
    }

    /// Set filters to refine search results beyond title matching.
    /// Use [`AnimeSearchBuilder::filter_match_mode`] to control how
    /// multiple filters are evaluated. Optional, no filtering by default.
    pub fn filter(mut self, filter: SearchFilter) -> Self {
        self.filter = Some(filter);
        self
    }

    /// If `true`, results are sorted alphabetically by title.
    /// Defaults to `false`.
    pub const fn sort(mut self, order: QuerySort) -> Self {
        self.config.sort = order;
        self
    }

    /// Set the offset for the page of queries
    pub const fn offset(mut self, offset: u32) -> Self {
        self.config.offset = offset;
        self
    }

    /// Set the status category of the user list read
    pub const fn status(mut self, status: Option<ListStatusEnum>) -> Self {
        self.config.status = status;
        self
    }

    pub async fn get(&self, user_name: Option<&str>) -> Result<Vec<UserAnimeListEdge>> {
        let uname: &str = user_name.unwrap_or(SELF_LIST);
        if uname == SELF_LIST && self.access_token.is_none() {
            return Err(Error::Unauthenticated);
        }

        let url_string: String = format!("{USERS_ENDPOINT}/{uname}/animelist");
        let mut query_params: HashMap<&str, compact_str::CompactString> = my_hash_map! {
            "sort" => self.config.sort,
            "limit" => self.config.limit,
            "offset" => self.config.offset
        };

        if let Some(status) = self.config.status.as_ref() {
            query_params.insert("status", compact_str::format_compact!("{status}"));
        }

        let url: Url = Url::parse_with_params(&url_string, query_params)?;
        let resp: Response = self.client.get(url).send().await?;
        if !resp.status().is_success() {
            return Err(Error::ResponseError);
        }

        let resp_val: serde_json::Value = resp.json().await?;
        let mut query_results: Vec<UserAnimeListEdge> = serde_json::from_value::<UserAnimeListQuery>(resp_val)?.data;
        query_results.retain(|d| self.filter.as_ref().is_none_or(|f| f.matches(&d.node, &self.search_mode)));

        Ok(query_results)
    }

    pub async fn update(&self, anime_id: u32, builder: UpdateBuilder, return_update: bool) -> Result<Option<ListStatus>> {
        if self.access_token.is_none() {
            return Err(Error::Unauthenticated);
        }

        let url_string: String = format!("{ANIME_ENDPOINT}/{anime_id}/my_list_status");
        let url: Url = Url::parse(&url_string)?;

        let resp: Response = self.client.put(url).form(&builder.into_params()).send().await?;
        if !resp.status().is_success() {
            return Err(Error::ResponseError);
        }

        let resp_val: serde_json::Value = resp.json().await?;
        let new_status: ListStatus = serde_json::from_value(resp_val)?;

        if return_update { Ok(Some(new_status)) } else { Ok(None) }
    }
}
