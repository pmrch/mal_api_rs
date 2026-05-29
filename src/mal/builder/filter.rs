use super::api::AnimeNode;
use super::models::{EpLengthMins, NumEps, SearchMode};

// A filter is just a function that takes an AnimeNode and returns bool
type AnimePredicate = Box<dyn Fn(&AnimeNode) -> bool + Send + Sync>;

#[derive(Default)]
pub struct SearchFilter {
    predicates:          Vec<AnimePredicate>,
    pub required_fields: Vec<&'static str>,
}

impl SearchFilter {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    #[must_use]
    pub fn min_score(mut self, score: f64) -> Self {
        self.required_fields.push("mean");
        self.predicates.push(Box::new(move |node| node.mean.is_some_and(|m| m.0 >= score)));
        self
    }

    #[must_use]
    pub fn media_type(mut self, mt: &'static str) -> Self {
        self.required_fields.push("media_type");
        self.predicates.push(Box::new(move |node| node.media_type.as_deref() == Some(mt)));
        self
    }

    #[must_use]
    pub fn genres(mut self, genres: &'static [&'static str]) -> Self {
        self.required_fields.push("genres");
        self.predicates.push(Box::new(move |node| {
            node.genres
                .as_ref()
                .is_some_and(|g| genres.iter().any(|genre| g.iter().any(|ng| ng.name.eq_ignore_ascii_case(genre))))
        }));

        self
    }

    #[must_use]
    pub fn started_after(mut self, date: chrono::NaiveDate) -> Self {
        self.required_fields.push("start_date");
        self.predicates
            .push(Box::new(move |node| node.start_date.is_some_and(|sd| sd >= date)));

        self
    }

    #[must_use]
    pub fn ended_before(mut self, date: chrono::NaiveDate) -> Self {
        self.required_fields.push("end_date");
        self.predicates
            .push(Box::new(move |node| node.end_date.is_some_and(|ed| ed <= date)));

        self
    }

    #[must_use]
    pub fn episodes(mut self, eps: NumEps) -> Self {
        self.required_fields.push("num_episodes");
        self.predicates
            .push(Box::new(move |node| node.num_episodes.is_some_and(|ne| eps.matches(ne))));

        self
    }

    #[must_use]
    pub fn episodes_len(mut self, ep_len: EpLengthMins) -> Self {
        self.required_fields.push("value");
        self.predicates.push(Box::new(move |node| {
            let ep_len_secs: Option<u32> = node.avg_ep_len;
            ep_len_secs.is_some_and(|esecs| ep_len.matches(esecs))
        }));

        self
    }

    fn matches_all(&self, node: &AnimeNode) -> bool { self.predicates.iter().all(|p| p(node)) }
    fn matches_any(&self, node: &AnimeNode) -> bool { self.predicates.iter().any(|p| p(node)) }

    #[must_use]
    pub fn matches(&self, node: &AnimeNode, search_mode: &SearchMode) -> bool {
        match search_mode {
            SearchMode::All => self.matches_all(node),
            SearchMode::Any => self.matches_any(node),
            SearchMode::AtLeast(num_match) => {
                if *num_match == 0 {
                    return true;
                }

                if *num_match > self.predicates.len() {
                    return false;
                }

                self.predicates.iter().filter(|p| p(node)).count() >= *num_match
            }
        }
    }
}
