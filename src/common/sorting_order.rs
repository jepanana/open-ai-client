use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// The order in which to sort the open ai results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortingOrder {
    /// Ascending
    #[serde(rename = "asc")]
    Ascending,

    /// Descending
    #[serde(rename = "desc")]
    Descending,
}

impl Display for SortingOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortingOrder::Ascending => write!(f, "asc"),
            SortingOrder::Descending => write!(f, "desc"),
        }
    }
}
