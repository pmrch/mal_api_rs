use super::{Arc, Client, CompactString, Error, Result, Url, UserInfo, check_response};

const URL: CompactString = CompactString::const_new("https://api.myanimelist.net/v2/users");

pub async fn get_user_info(client: &Arc<Client>, access_token: Option<Arc<str>>, user_id: usize) -> Result<UserInfo> {
    let Some(token) = access_token else {
        return Err(Error::Unauthorized);
    };

    let url: Url = Url::parse(&format!("{URL}/{user_id}"))?;
    let resp: reqwest::Response = client.get(url).bearer_auth(token).send().await?;
    check_response(resp.status())?;

    let resp_val: serde_json::Value = resp.json().await?;
    Ok(serde_json::from_value(resp_val)?)
}
