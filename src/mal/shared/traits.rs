use super::api::{AlternativeTitles, AnimeNode};

pub trait HasNode {
    fn node(&self) -> &AnimeNode;
}

pub trait Name {
    fn name(&self) -> &str;
}

pub trait HasTitles {
    fn title(&self) -> &str;
    fn alternative_titles(&self) -> &Option<AlternativeTitles>;
}
