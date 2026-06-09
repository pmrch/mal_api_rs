use chrono::NaiveDate;
use serde::Deserialize;

use super::models::SortOrder;
use super::shared::traits::{HasNode, HasTitles};
use super::{Error, Result};

pub(super) fn is_relevant(title: impl Into<String>, query: &str, threshold: f64) -> bool {
    strsim::jaro_winkler(&title.into().to_lowercase(), &query.to_lowercase()) >= threshold
}

pub(super) fn matches_title<T: HasTitles>(node: &T, title: &str, threshold: f64) -> bool {
    let title: String = title.to_lowercase();
    let words: Vec<&str> = title.split_whitespace().collect();
    let query_title_lower: String = node.title().to_lowercase();

    let words_match: bool = words.iter().any(|word| query_title_lower.contains(word));
    let words_match_alt: bool = words.iter().any(|word| {
        node.alternative_titles().as_ref().is_some_and(|a| {
            a.en.to_lowercase().contains(word)
                || a.ja.to_lowercase().contains(word)
                || a.synonyms.iter().any(|s| s.to_lowercase().contains(word))
        })
    });

    let is_alternative_title: bool = node.alternative_titles().as_ref().is_some_and(|a_title| {
        is_relevant(&title, &a_title.en, threshold)
            || is_relevant(&title, &a_title.ja, threshold)
            || a_title.synonyms.iter().any(|syn| is_relevant(&title, syn, threshold))
    });

    (is_relevant(title, node.title(), threshold) && words_match) || (is_alternative_title && words_match_alt)
}

pub(super) fn deserialize_date<'de, D>(deserializer: D) -> std::result::Result<Option<chrono::NaiveDate>, D::Error>
where D: serde::Deserializer<'de> {
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(date_str) = s {
        if date_str.is_empty() {
            eprintln!("MAL returned empty date string");
            return Ok(None);
        }

        let parsed: NaiveDate = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .or_else(|_| NaiveDate::parse_from_str(&format!("{date_str}-01"), "%Y-%m-%d"))
            .or_else(|_| NaiveDate::parse_from_str(&format!("{date_str}-01-01"), "%Y-%m-%d"))
            .map_err(serde::de::Error::custom)?;

        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

pub(super) fn check_response(status: reqwest::StatusCode) -> Result<()> {
    match status {
        s if s.is_success() => Ok(()),
        reqwest::StatusCode::UNAUTHORIZED => Err(Error::Unauthorized),
        reqwest::StatusCode::NOT_FOUND => Err(Error::NotFound),
        reqwest::StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimited),
        s if s.is_server_error() => Err(Error::ServerError(s)),
        s => Err(Error::ResponseError(s)),
    }
}

/// Sort a slice of anime by the configured sort order.
pub(super) fn sort_vec<T: HasNode>(sort_order: Option<&SortOrder>, input: &mut [T]) {
    if let Some(sort_order) = &sort_order {
        input.sort_by(|a, b| match sort_order {
            SortOrder::Title => a.node().title.cmp(&b.node().title),
            SortOrder::MeanScore => a.node().mean.cmp(&b.node().mean),
            SortOrder::StartDate => a.node().start_date.cmp(&b.node().start_date),
            SortOrder::Popularity => a.node().popularity.cmp(&b.node().popularity),
            SortOrder::Rank => a.node().rank.cmp(&b.node().rank),
        });
    }
}
