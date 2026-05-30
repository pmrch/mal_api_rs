use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),

    #[error(".env file read error: {0}")]
    Dotenv(#[from] dotenv::Error),

    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),

    #[error("Invalid Header Value error: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Int parsing error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Standard I?o error: {0}")]
    StdIo(#[from] std::io::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Unauthorized: invalid or missing access token")]
    Unauthorized, // 401

    #[error("Resource not found")]
    NotFound, // 404

    #[error("Rate limit exceeded, slow down")]
    RateLimited, // 429

    #[error("MAL server error: {0}")]
    ServerError(reqwest::StatusCode), // 5xx

    #[error("Unexpected status code: {0}")]
    ResponseError(reqwest::StatusCode), // everything else
}
