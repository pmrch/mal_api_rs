use compact_str::CompactString;

use super::traits::{HasNode, Name};

impl Name for super::api::Genre {
    fn name(&self) -> CompactString { self.name.as_str().into() }
}

impl Name for super::api::Anime {
    fn name(&self) -> CompactString { self.node.title.as_str().into() }
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
