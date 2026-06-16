use super::endpoints::MANGA_ENDPOINT;
use super::helpers::{check_response, matches_title};
use super::models::{MangaNode, MangaQuery, MangaRankingQuery, MangaRankingQueryData, MangaRankingType, MangaSearchFilter};
use super::requests::{Client, Response};
use super::shared_models::{SearchConfig, SearchMode, SortOrder};
use super::{Arc, CompactString, HashMap, HashSet, MangaHasNode, Result, Url, my_hash_map};

pub struct MangaSearchBuilder {
    client:        Arc<Client>,
    config:        SearchConfig,
    search_mode:   SearchMode,
    search_filter: Option<MangaSearchFilter>,
    sorting_order: Option<SortOrder>,
}

impl MangaSearchBuilder {
    #[must_use]
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            config: SearchConfig::default(),
            search_mode: SearchMode::All,
            search_filter: None,
            sorting_order: None,
        }
    }

    #[must_use]
    /// Set the similarity threshold for title comparison with query
    pub const fn threshold(mut self, threshold: f64) -> Self {
        self.config.threshold = threshold;
        self
    }

    #[must_use]
    /// Set the number of matches you wish to receive
    pub const fn number_of_titles(mut self, num_titles: usize) -> Self {
        self.config.num_titles = num_titles;
        self
    }

    #[must_use]
    /// Set the limit of anime per page
    pub const fn limit(mut self, limit: u32) -> Self {
        self.config.limit = limit;
        self
    }

    #[must_use]
    /// If `true`, results are sorted alphabetically by title.
    /// Defaults to `false`.
    pub const fn sort(mut self, order: SortOrder) -> Self {
        self.sorting_order = Some(order);
        self
    }

    #[must_use]
    /// Set the offset for the page of queries
    pub const fn first_page_offset(mut self, offset: u32) -> Self {
        self.config.first_page_offset = offset;
        self
    }

    #[must_use]
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

    #[must_use]
    /// Set filters to refine search results beyond title matching.
    /// Use [`AnimeSearchBuilder::filter_match_mode`] to control how
    /// multiple filters are evaluated. Optional, no filtering by default.
    pub fn filter(mut self, filter: MangaSearchFilter) -> Self {
        self.search_filter = Some(filter);
        self
    }

    #[must_use]
    /// Set the fields that should get queried and deserialized
    pub fn fields<S: AsRef<str>, I: Iterator<Item = S>>(mut self, fields: I) -> Self {
        self.config.fields = fields.into_iter().map(|item| CompactString::from(item.as_ref())).collect();
        self
    }

    /// Search any number of manga by title, and set parameters of fields,
    /// number of titles, threshold, page limit
    ///
    /// # Errors
    ///
    /// - Returns an error if URL parsing fails
    /// - Returns an error if the rquest fails
    /// - Returns an error if response can't be parsed as JSON
    pub async fn search(&mut self, title: impl AsRef<str>) -> Result<Vec<MangaNode>> {
        self.build_fields();
        let query_params: HashMap<&str, CompactString> = crate::my_hash_map! {
            "q" => title.as_ref(),
            "offset" => self.config.first_page_offset,
            "limit" => self.config.limit,
            "nsfw" => "true",
            "fields" => &self.config.fields.join(",")
        };

        let url: Url = Url::parse_with_params(&MANGA_ENDPOINT, query_params)?;
        let response: Response = self.client.get(url).send().await?;
        check_response(response.status())?;

        let mut target_query: HashSet<MangaNode> = HashSet::with_capacity(self.config.num_titles);
        let mut pages_without_match: usize = 0;

        let resp_val: serde_json::Value = response.json().await?;
        let mut response: MangaQuery = serde_json::from_value(resp_val)?;
        println!("OI");

        let mut current_limit: u32 = 0;
        loop {
            if current_limit == self.config.limit {
                break;
            }

            current_limit += 1;
            for entry in response.data {
                let filter_matches: bool = self.search_filter.as_ref().is_none_or(|f| f.matches(&entry.node, &self.search_mode));
                if matches_title(&entry, title.as_ref(), self.config.threshold) && filter_matches {
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

            response = resp.json::<MangaQuery>().await?;
            pages_without_match += 1;
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }

        let mut mangas: Vec<MangaNode> = target_query.into_iter().collect();
        self.sort_vec(&mut mangas);

        Ok(mangas)
    }

    /// Retrieves requested details about the manga
    ///
    /// # Errors
    ///
    /// - Returns an error if the URL can't be parsed
    /// - Returns an error if the response status code is an error
    /// - Returns an error if deserialization fails
    pub async fn details(&self, manga_id: u32) -> Result<MangaNode> {
        let url_string: String = format!("{MANGA_ENDPOINT}/{manga_id}");
        let url: Url = Url::parse_with_params(&url_string, &crate::my_hash_map! {"fields" => self.config.fields.join(",")})?;
        let resp: Response = self.client.get(url).send().await?;
        check_response(resp.status())?;

        let resp_val: serde_json::Value = resp.json().await?;
        Ok(serde_json::from_value(resp_val)?)
    }

    /// Retrieves requested details about the manga
    ///
    /// # Errors
    ///
    /// - Returns an error if the URL can't be parsed
    /// - Returns an error if the response status code is an error
    /// - Returns an error if deserialization fails
    pub async fn ranking(&self, ranking_type: MangaRankingType) -> Result<Vec<MangaRankingQueryData>> {
        let url_string: String = format!("{MANGA_ENDPOINT}/ranking");
        let params = my_hash_map! {
            "ranking_type" => ranking_type,
            "limit" => self.config.limit,
            "offset" => self.config.first_page_offset,
            "fields" => self.config.fields.join(",")
        };

        let url: Url = Url::parse_with_params(&url_string, params)?;
        let resp: Response = self.client.get(url).send().await?;
        check_response(resp.status())?;

        let resp_val: serde_json::Value = resp.json().await?;
        let resp_obj: MangaRankingQuery = serde_json::from_value(resp_val)?;
        let mut data: Vec<MangaRankingQueryData> = resp_obj.data;

        data.retain(|d| self.search_filter.as_ref().is_none_or(|f| f.matches(&d.node, &self.search_mode)));
        self.sort_vec(&mut data);
        Ok(data)
    }

    /// Sort a slice of anime by the configured sort order.
    fn sort_vec<T: MangaHasNode>(&self, input: &mut [T]) {
        if let Some(sort_order) = &self.sorting_order {
            input.sort_by(|a, b| match sort_order {
                SortOrder::Title => a.node().title.cmp(&b.node().title),
                SortOrder::MeanScore => a.node().mean.cmp(&b.node().mean),
                SortOrder::StartDate => a.node().start_date.cmp(&b.node().start_date),
                SortOrder::Popularity => a.node().popularity.cmp(&b.node().popularity),
                SortOrder::Rank => a.node().rank.cmp(&b.node().rank),
            });
        }
    }

    fn build_fields(&mut self) {
        if let Some(ord) = &self.sorting_order
            && let Some(req_field) = ord.required_field()
            && !self.config.fields.contains(&req_field)
        {
            self.config.fields.push(req_field);
        }

        if let Some(filter) = &self.search_filter {
            for field in &filter.required_fields {
                let field: CompactString = CompactString::const_new(field);
                if !self.config.fields.contains(&field) {
                    self.config.fields.push(field);
                }
            }
        }
    }
}
