use super::config::UserAnimeConfig;
use super::custom::{QuerySort, SearchMode};
use super::helpers::check_response;
use super::models::{ListStatus, ListStatusEnum, UserAnimeListEdge, UserAnimeListQuery};
use super::{Arc, Client, Error, HashMap, Response, Result, SearchFilter, UpdateBuilder, Url};

const ANIME_ENDPOINT: &str = "https://api.myanimelist.net/v2/anime";
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

    /// Retrieve the anime list of a MAL user.
    ///
    /// If `user_name` is `None`, defaults to the authenticated user's list
    /// (`@me`). Requires authentication when fetching `@me`.
    ///
    /// Filters set via [`UserAnimeBuilder::filter`] are applied client-side
    /// after fetching.
    ///
    /// # Errors
    /// - [`Error::Unauthenticated`] if no access token is set and `user_name`
    ///   is `None`
    /// - [`Error::ResponseError`] if MAL returns a non-success status code
    pub async fn get(&self, user_name: Option<&str>) -> Result<Vec<UserAnimeListEdge>> {
        let uname: &str = user_name.unwrap_or(SELF_LIST);
        let url_string: String = format!("{USERS_ENDPOINT}/{uname}/animelist");
        let mut query_params: HashMap<&str, compact_str::CompactString> = crate::my_hash_map! {
            "sort" => self.config.sort,
            "limit" => self.config.limit,
            "offset" => self.config.offset
        };

        if let Some(status) = self.config.status.as_ref() {
            query_params.insert("status", compact_str::format_compact!("{status}"));
        }

        let url: Url = Url::parse_with_params(&url_string, query_params)?;
        let resp: Response = if let Some(token) = &self.access_token {
            self.client.get(url).bearer_auth(token).send().await?
        } else {
            if uname == SELF_LIST {
                return Err(Error::Unauthorized);
            }

            self.client.get(url).send().await?
        };

        check_response(resp.status())?;
        let resp_val: serde_json::Value = resp.json().await?;
        let mut query_results: Vec<UserAnimeListEdge> = serde_json::from_value::<UserAnimeListQuery>(resp_val)?.data;

        tracing::info!(
            user = %uname,
            results = %query_results.len(),
            filtered = %(self.filter.is_some()),
            "User anime list fetched successfully"
        );

        query_results.retain(|d| self.filter.as_ref().is_none_or(|f| f.matches(&d.node, &self.search_mode)));
        Ok(query_results)
    }

    /// Update an anime entry on the authenticated user's MAL list.
    ///
    /// Only fields explicitly set on the [`UpdateBuilder`] will be modified,
    /// all other fields remain unchanged on MAL's side.
    ///
    /// # Errors
    /// - [`Error::Unauthenticated`] if no access token is set
    /// - [`Error::ResponseError`] if MAL returns a non-success status code
    pub async fn update(&self, anime_id: u32, builder: UpdateBuilder) -> Result<()> {
        self.update_inner(anime_id, builder).await?;
        tracing::info!(
            anime_id = %anime_id,
            "Anime list entry updated successfully"
        );

        Ok(())
    }

    /// Update an anime entry and return the updated [`ListStatus`] from MAL.
    ///
    /// Identical to [`UserAnimeBuilder::update`] but deserializes and returns
    /// the updated list status from MAL's response.
    ///
    /// # Errors
    /// - [`Error::Unauthenticated`] if no access token is set
    /// - [`Error::ResponseError`] if MAL returns a non-success status code
    pub async fn update_and_return(&self, anime_id: u32, builder: UpdateBuilder) -> Result<ListStatus> {
        let new_status: ListStatus = self.update_inner(anime_id, builder).await?;
        tracing::info!(
            anime_id = %anime_id,
            new_status = %new_status.status,
            new_score = %new_status.score,
            "Anime list entry updated successfully"
        );

        Ok(new_status)
    }

    async fn update_inner(&self, anime_id: u32, builder: UpdateBuilder) -> Result<ListStatus> {
        let url_string: String = format!("{ANIME_ENDPOINT}/{anime_id}/my_list_status");
        let url: Url = Url::parse(&url_string)?;
        let resp: Response = if let Some(token) = &self.access_token {
            self.client.put(url).form(&builder.into_params()).bearer_auth(token).send().await?
        } else {
            return Err(Error::Unauthorized);
        };

        tracing::info!(
            anime_id = %anime_id,
            "List entry update request sent"
        );

        check_response(resp.status())?;
        let resp_val: serde_json::Value = resp.json().await?;
        let new_status: ListStatus = serde_json::from_value(resp_val)?;

        Ok(new_status)
    }

    /// Delete an anime entry from the authenticated user's MAL list.
    ///
    /// This action is irreversible — the entry will be permanently removed
    /// from the user's list on MAL.
    ///
    /// # Errors
    /// - [`Error::Unauthenticated`] if no access token is set
    /// - [`Error::ResponseError`] if MAL returns a non-success status code
    pub async fn delete(&self, anime_id: u32) -> Result<()> {
        let url_string: String = format!("{ANIME_ENDPOINT}/{anime_id}/my_list_status");
        let url: Url = Url::parse(&url_string)?;
        let resp: Response = if let Some(token) = &self.access_token {
            self.client.delete(url).bearer_auth(token).send().await?
        } else {
            return Err(Error::Unauthorized);
        };

        check_response(resp.status())?;
        tracing::info!(
            anime_id = %anime_id,
            "Anime list entry deleted successfully"
        );

        Ok(())
    }
}
