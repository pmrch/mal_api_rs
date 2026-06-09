use super::config::UserMangeConfig;
use super::endpoints::MANGA_ENDPOINT;
use super::helpers::check_response;
use super::requests::{Client, Request, Response};
use super::shared_models::SearchMode;
use super::update::MangaUpdateBuilder;
use super::{Arc, CompactString, Error, Result, Url, format_compact};

#[rustfmt::skip]
use super::models::{
    MangaListStatus, MangaListStatusEnum, MangaNode, MangaQuerySort, MangaSearchFilter, UserMangaListEdge, UserMangaListQuery,
};

const SELF_LIST: &str = "@me";
const USERS_ENDPOINT: &str = "https://api.myanimelist.net/v2/users";

pub struct UserMangaSearchBuilder {
    client:       Arc<Client>,
    access_token: Option<Arc<str>>,
    filter:       Option<MangaSearchFilter>,
    config:       UserMangeConfig,
    search_mode:  SearchMode,
}

impl UserMangaSearchBuilder {
    pub fn new(client: Arc<Client>, access_token: Option<Arc<str>>) -> Self {
        Self {
            client,
            access_token,
            filter: None,
            config: UserMangeConfig::default(),
            search_mode: SearchMode::All,
        }
    }

    /// Set the limit of anime per page
    pub const fn limit(mut self, limit: u32) -> Self {
        self.config.limit = limit;
        self
    }

    /// If `true`, results are sorted alphabetically by title.
    /// Defaults to `false`.
    pub const fn sort(mut self, order: MangaQuerySort) -> Self {
        self.config.sort = order;
        self
    }

    /// Set the offset for the page of queries
    pub const fn offset(mut self, offset: u32) -> Self {
        self.config.offset = offset;
        self
    }

    /// Set the status category of the user list read
    pub fn status(mut self, status: Option<MangaListStatusEnum>) -> Self {
        self.config.status = status.unwrap_or_default();
        self
    }

    /// Set filters to refine search results beyond title matching.
    /// Use [`AnimeSearchBuilder::filter_match_mode`] to control how
    /// multiple filters are evaluated. Optional, no filtering by default.
    pub fn filter(mut self, filter: MangaSearchFilter) -> Self {
        self.filter = Some(filter);
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

    pub async fn get(&self, user_name: Option<impl Into<CompactString>>) -> Result<Vec<UserMangaListEdge>> {
        let user_name: CompactString = user_name.map_or(CompactString::from(SELF_LIST), |s| s.into());
        let url_string: CompactString = format_compact!("{USERS_ENDPOINT}/{user_name}/mangalist");
        let url: Url = Url::parse(&url_string)?;

        let resp: Response = if let Some(token) = &self.access_token {
            self.client.get(url).bearer_auth(token).send().await?
        } else {
            if user_name == SELF_LIST {
                return Err(Error::Unauthorized);
            }

            self.client.get(url).send().await?
        };

        check_response(resp.status())?;
        let resp_val: serde_json::Value = resp.json().await?;
        let mut query_results: Vec<UserMangaListEdge> = serde_json::from_value::<UserMangaListQuery>(resp_val)?.data;

        tracing::info!(
            user = %user_name,
            results = %query_results.len(),
            filtered = %(self.filter.is_some()),
            "User manga list fetched successfully"
        );

        query_results.retain(|d| self.filter.as_ref().is_none_or(|f| f.matches(&d.node, &self.search_mode)));
        Ok(query_results)
    }

    pub async fn delete(&self, manga_id: u32) -> Result<()> {
        let url_string: CompactString = format_compact!("{MANGA_ENDPOINT}/{manga_id}/my_list_status");
        let url: Url = Url::parse(&url_string)?;

        let resp: Response = if let Some(token) = &self.access_token {
            self.client.delete(url).bearer_auth(token).send().await?
        } else {
            return Err(Error::Unauthorized);
        };

        check_response(resp.status())?;
        tracing::info!(
            manga_id = %manga_id,
            "Anime list entry deleted successfully"
        );

        Ok(())
    }

    pub async fn update(&self, manga_id: u32, builder: MangaUpdateBuilder) -> Result<()> {
        self.update_inner(manga_id, builder).await?;
        tracing::info!(
            manga_id = %manga_id,
            "Manga list entry updated successfully"
        );

        Ok(())
    }

    pub async fn update_and_return(&self, manga_id: u32, builder: MangaUpdateBuilder) -> Result<MangaListStatus> {
        self.update_inner(manga_id, builder).await
    }

    async fn update_inner(&self, manga_id: u32, builder: MangaUpdateBuilder) -> Result<MangaListStatus> {
        let url_string: CompactString = format_compact!("{MANGA_ENDPOINT}/{manga_id}/my_list_status");
        let url: Url = Url::parse(&url_string)?;

        let request: Request = if let Some(access_token) = &self.access_token {
            self.client.put(url).bearer_auth(access_token).form(&builder.into_params()).build()?
        } else {
            return Err(Error::Unauthorized);
        };

        let response: Response = self.client.execute(request).await?;
        check_response(response.status())?;

        let resp_val: serde_json::Value = response.json().await?;
        let resp: MangaListStatus = serde_json::from_value(resp_val)?;

        Ok(resp)
    }
}
