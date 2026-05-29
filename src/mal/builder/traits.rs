use super::api::AnimeNode;

pub trait HasNode {
    fn node(&self) -> &AnimeNode;
}
