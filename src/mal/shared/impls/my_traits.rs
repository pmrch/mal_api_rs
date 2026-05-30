use super::traits::{HasNode, Name};

impl Name for super::api::Genre {
    fn name(&self) -> &str { self.name.as_str() }
}

impl Name for super::api::Anime {
    fn name(&self) -> &str { &self.node.title }
}

impl HasNode for super::api::AnimeNode {
    fn node(&self) -> &crate::prelude::AnimeNode { self }
}

impl HasNode for super::api::RankingQueryData {
    fn node(&self) -> &crate::prelude::AnimeNode { &self.node }
}

impl HasNode for super::api::Anime {
    fn node(&self) -> &crate::prelude::AnimeNode { &self.node }
}
