use super::models::MangaSearchBuilder;
use super::requests::{Client, ClientBuilder, HeaderMap, HeaderValue, Policy};
use super::sync::Arc;
use super::{AnimeSearchBuilder, Result, UserAnimeBuilder};

#[rustfmt::skip]
#[cfg(feature = "user")]
use super::get_user_info;

#[cfg(feature = "user")]
use super::models::UserInfo;

pub struct MalApi {
    client:       Arc<Client>,
    access_token: Option<Arc<str>>,
}

impl MalApi {
    /// Creates a new instance of the MAL API
    ///
    /// # Errors
    ///
    /// - Returns an error if invalid header value was provided for client ID or
    ///   auth token
    /// - Returns an error if `reqwest::ClientBuilder` fails to construct the
    ///   client
    pub fn new(access_token: Option<&str>, client_id: &str) -> Result<Self> {
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert("X-MAL-CLIENT-ID", HeaderValue::from_str(client_id)?);

        let client: Client = ClientBuilder::new()
            .redirect(Policy::limited(2))
            .user_agent("MAL_API_RS")
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(5))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client:       Arc::new(client),
            access_token: access_token.map(Arc::from),
        })
    }

    #[must_use]
    pub fn anime(&self) -> AnimeSearchBuilder { AnimeSearchBuilder::new(self.client.clone(), self.access_token.as_ref().map(Arc::clone)) }

    #[must_use]
    pub fn user_anime(&self) -> UserAnimeBuilder {
        UserAnimeBuilder::new(self.client.clone(), self.access_token.as_ref().map(Arc::clone))
    }

    #[must_use]
    pub fn manga(&self) -> MangaSearchBuilder { MangaSearchBuilder::new(self.client.clone()) }

    #[cfg(feature = "user")]
    /// This function retrives user informations based on user ID
    ///
    /// # Errors
    /// - Returns an error if the URL failed to parse
    /// - Returns an error if the request failed ore received error for status
    pub async fn get_user(&self, user_id: usize) -> Result<UserInfo> {
        get_user_info(&self.client, self.access_token.as_ref().map(Arc::clone), user_id).await
    }
}
