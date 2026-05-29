use super::Arc;
use super::requests::Client;

const USERS_ENDPOINT: &str = "https://api.myanimelist.net/v2/users";

pub struct UserAnimeBuilder {
    client:       Arc<Client>,
    access_token: Option<Arc<str>>,
}

impl UserAnimeBuilder {
    pub fn new(client: Arc<Client>, access_token: Option<Arc<str>>) -> Self { Self { client, access_token } }
}
