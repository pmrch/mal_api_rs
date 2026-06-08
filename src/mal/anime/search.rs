use super::custom::{SearchConfig, SearchMode, SortOrder};
use super::endpoints::{ANIME_ENDPOINT, RANKING_ENDPOINT, SEASONAL_ENDPOINT, SUGGESTION_ENDPOINT};
use super::helpers::{check_response, matches_title, sort_vec};
use super::models::{Anime, AnimeNode, AnimeQuery, AnimeRankingType, RankingQuery, RankingQueryData, SeasonEnum};
use super::{Arc, Client, CompactString, Error, HasNode, HashMap, HashSet, Response, Result, SearchFilter, Url};

pub struct AnimeSearchBuilder {
    client:        Arc<Client>,
    config:        SearchConfig,
    access_token:  Option<Arc<str>>,
    sorting_order: Option<SortOrder>,
    search_filter: Option<SearchFilter>,
    search_mode:   SearchMode,
}

impl AnimeSearchBuilder {
    pub fn new(client: Arc<Client>, access_token: Option<Arc<str>>) -> Self {
        let config: SearchConfig = SearchConfig::default();

        Self {
            client,
            config,
            access_token,
            sorting_order: None,
            search_filter: None,
            search_mode: SearchMode::All,
        }
    }

    /// Set the similarity threshold for title comparison with query
    pub const fn threshold(mut self, threshold: f64) -> Self {
        self.config.threshold = threshold;
        self
    }

    /// Set the number of matches you wish to receive
    pub const fn number_of_titles(mut self, num_titles: usize) -> Self {
        self.config.num_titles = num_titles;
        self
    }

    /// Set the limit of anime per page
    pub const fn limit(mut self, limit: u32) -> Self {
        self.config.limit = limit;
        self
    }

    /// If `true`, results are sorted alphabetically by title.
    /// Defaults to `false`.
    pub const fn sort(mut self, order: SortOrder) -> Self {
        self.sorting_order = Some(order);
        self
    }

    /// Set the offset for the page of queries
    pub const fn first_page_offset(mut self, offset: u32) -> Self {
        self.config.first_page_offset = offset;
        self
    }

    /// Controls how multiple filters are evaluated against each result.
    /// - [`SearchMode::All`] — all filters must match (AND logic)
    /// - [`SearchMode::Any`] — at least one filter must match (OR logic)
    /// - [`SearchMode::AtLeast(n)`] — at least `n` filters must match
    ///
    /// Optional, defaults to [`SearchMode::All`]
    pub const fn filter_match_mode(mut self, search_mode: SearchMode) -> Self {
        self.search_mode = search_mode;
        self
    }

    /// Set filters to refine search results beyond title matching.
    /// Use [`AnimeSearchBuilder::filter_match_mode`] to control how
    /// multiple filters are evaluated. Optional, no filtering by default.
    pub fn filter(mut self, filter: SearchFilter) -> Self {
        self.search_filter = Some(filter);
        self
    }

    /// Set the fields that should get queried and deserialized
    pub fn fields<S: AsRef<str>, I: Iterator<Item = S> + Clone>(mut self, fields: I) -> Self {
        self.config.fields = fields.into_iter().map(|i| CompactString::new(i.as_ref())).collect();
        self
    }

    /// Search any number of anime by title, and set parameters of fields,
    /// number of titles, threshold, page limit
    pub async fn search(&self, title: &str) -> Result<Vec<AnimeNode>> {
        let new_fields: Vec<&str> = self.build_fields();
        let query_params: HashMap<&str, compact_str::CompactString> = crate::my_hash_map! {
            "q" => title,
            "offset" => self.config.first_page_offset,
            "limit" => self.config.limit,
            "nsfw" => "true",
            "fields" => &new_fields.join(",")
        };

        let url: Url = Url::parse_with_params(ANIME_ENDPOINT, query_params)?;
        let response: Response = self.client.get(url).send().await?;
        check_response(response.status())?;

        let mut target_query: HashSet<AnimeNode> = HashSet::with_capacity(self.config.num_titles);
        let mut pages_without_match: usize = 0;

        let resp_val: serde_json::Value = response.json().await?;
        let mut response: AnimeQuery = serde_json::from_value(resp_val)?;

        loop {
            for entry in response.data {
                let filter_matches: bool = self.search_filter.as_ref().is_none_or(|f| f.matches(&entry.node, &self.search_mode));

                if matches_title(&entry.node, title, self.config.threshold) && filter_matches {
                    pages_without_match = 0;
                    if target_query.len() >= self.config.num_titles {
                        break;
                    }

                    target_query.insert(entry.node);
                }
            }

            if target_query.len() >= self.config.num_titles || pages_without_match >= 8 {
                break;
            }

            let next_page = match &response.paging.next {
                Some(next) => {
                    if next.is_empty() {
                        break;
                    }

                    next
                }
                None => break,
            };

            let Ok(url_maybe_last) = url::Url::parse(next_page) else {
                eprintln!("MAL returned malformed next page URL: {next_page}");
                break;
            };

            let resp: Response = self.client.get(url_maybe_last).send().await?;
            if check_response(resp.status()).is_err() {
                tracing::error!("An error response was received for a page");
                break;
            }

            response = resp.json::<AnimeQuery>().await?;
            pages_without_match += 1;
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        let mut animes: Vec<AnimeNode> = target_query.into_iter().collect();
        sort_vec(self.sorting_order.as_ref(), &mut animes);

        Ok(animes)
    }

    /// Search anime by ID, from the builder struct you only have to set fields,
    /// other parameters will take no effect
    pub async fn details(&self, anime_id: u32) -> Result<AnimeNode> {
        let new_fields: Vec<&str> = self.build_fields();
        let url: Url = Url::parse_with_params(&format!("{ANIME_ENDPOINT}/{anime_id}"), &[("fields", &new_fields.join(","))])?;

        let resp: Response = self.client.get(url).send().await?;
        check_response(resp.status())?;
        Ok(resp.json::<Anime>().await?.node)
    }

    /// For this endpoint you may construct `limit()` and `fields()`, as well as
    /// `filter()`.
    ///
    /// This endpoint returns the top ranked animes
    pub async fn ranking(&self, ranking_type: AnimeRankingType) -> Result<Vec<RankingQueryData>> {
        let new_fields: Vec<&str> = self.build_fields();
        let query_params: HashMap<&str, compact_str::CompactString> = crate::my_hash_map! {
            "ranking_type" => ranking_type.as_ref(),
            "fields" => &new_fields.join(","),
            "limit" => self.config.limit,
            "offset" => self.config.first_page_offset
        };

        let url: Url = Url::parse_with_params(RANKING_ENDPOINT, query_params)?;
        let resp: Response = self.client.get(url).send().await?;
        check_response(resp.status())?;

        let resp_val: serde_json::Value = resp.json().await?;
        let mut query_data: Vec<RankingQueryData> = serde_json::from_value::<RankingQuery>(resp_val)?.data;

        query_data.retain(|d| self.search_filter.as_ref().is_none_or(|f| f.matches(&d.node, &self.search_mode)));
        sort_vec(self.sorting_order.as_ref(), &mut query_data);

        Ok(query_data)
    }

    /// Fetch anime airing in a specific season.
    ///
    /// Results are filtered and sorted according to builder configuration.
    pub async fn seasonal(&self, year: u16, season: SeasonEnum) -> Result<Vec<Anime>> {
        let new_fields: Vec<&str> = self.build_fields();
        let query_params: HashMap<&str, compact_str::CompactString> = crate::my_hash_map! {
            "limit" => self.config.limit,
            "offset" => self.config.first_page_offset,
            "fields" => &new_fields.join(",")
        };

        let url: Url = Url::parse_with_params(&format!("{SEASONAL_ENDPOINT}/{year}/{season}"), query_params)?;
        let resp: Response = self.client.get(url).send().await?;
        check_response(resp.status())?;

        let resp_val: serde_json::Value = resp.json().await?;
        let mut query_results: Vec<Anime> = serde_json::from_value::<AnimeQuery>(resp_val)?.data;

        query_results.retain(|r| self.search_filter.as_ref().is_none_or(|f| f.matches(r.node(), &self.search_mode)));
        sort_vec(self.sorting_order.as_ref(), &mut query_results);

        Ok(query_results)
    }

    /// Fetch personalized anime suggestions (requires authentication).
    ///
    /// Returns recommendations based on the authenticated user's preferences.
    /// Results are filtered and sorted according to builder configuration.
    pub async fn suggestions(&self) -> Result<Vec<Anime>> {
        if self.access_token.is_none() {
            return Err(Error::Unauthorized);
        }

        let new_fields: Vec<&str> = self.build_fields();
        let query_params: HashMap<&str, compact_str::CompactString> = crate::my_hash_map! {
            "limit" => self.config.limit,
            "offset" => self.config.first_page_offset,
            "fields" => &new_fields.join(",")
        };

        let url: Url = Url::parse_with_params(SUGGESTION_ENDPOINT, query_params)?;
        let resp: reqwest::Response = self.client.get(url).send().await?;
        check_response(resp.status())?;

        let resp_val: serde_json::Value = resp.json().await?;
        let mut query_results: Vec<Anime> = serde_json::from_value::<AnimeQuery>(resp_val)?.data;

        query_results.retain(|r| self.search_filter.as_ref().is_none_or(|f| f.matches(r.node(), &self.search_mode)));
        sort_vec(self.sorting_order.as_ref(), &mut query_results);

        Ok(query_results)
    }

    /// Build the complete field set for API queries.
    ///
    /// Automatically injects required fields for sorting and filtering.
    fn build_fields(&self) -> Vec<&str> {
        let mut fields: Vec<_> = self.config.fields.clone();
        if let Some(ord) = &self.sorting_order
            && let Some(req_field) = ord.required_field()
            && !self.config.fields.contains(&req_field)
        {
            fields.push(req_field);
        }

        if let Some(filter) = &self.search_filter {
            for field in &filter.required_fields {
                let field: CompactString = CompactString::const_new(field);
                if !fields.contains(&field) {
                    fields.push(field);
                }
            }
        }

        self.config.fields.iter().map(CompactString::as_str).collect()
    }
}
