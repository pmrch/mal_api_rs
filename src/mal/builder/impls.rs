use super::HasNode;
use super::api::{Anime, AnimeNode, RankingQueryData};

impl HasNode for AnimeNode {
    fn node(&self) -> &Self { self }
}

impl HasNode for Anime {
    fn node(&self) -> &AnimeNode { &self.node }
}

impl HasNode for RankingQueryData {
    fn node(&self) -> &AnimeNode { &self.node }
}
