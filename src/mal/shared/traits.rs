use super::api::AnimeNode;

pub trait HasNode {
    fn node(&self) -> &AnimeNode;
}

pub trait Name {
    fn name(&self) -> &str;
}
