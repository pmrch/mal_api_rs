use super::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub mod sync {
    pub use std::sync::Arc;
}

pub use std::collections::{HashMap, HashSet};

pub use crate::mal::{MalApi, NumEps, SearchFilter, SortOrder, models};
pub use crate::my_hash_map;
