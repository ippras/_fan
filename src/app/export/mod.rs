use polars::prelude::DataFrame;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// MetaDataFrame
// #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MetaDataFrame {
    pub meta: BTreeMap<String, String>,
    pub data: DataFrame,
}

impl MetaDataFrame {
    pub const fn new(meta: BTreeMap<String, String>, data: DataFrame) -> Self {
        Self { meta, data }
    }
}

// pub mod parquet;
// pub mod xlsx;
pub mod ron;
