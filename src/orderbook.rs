#[path = "utils.rs"] mod utils;
use std::cmp::Reverse;
use ordered_float::NotNan;
use std::collections::BTreeMap;

type MinNonNan = Reverse<NotNan<f64>>;

#[derive(Debug)]
pub struct Book {
    pub buy_tree: BTreeMap<MinNonNan, Limit>,
    pub sell_tree: BTreeMap<MinNonNan, Limit>
}

#[derive(Debug, Clone, Default)]
pub struct Limit {
    pub limit_price: MinNonNan,
    pub total_volume: MinNonNan
}

impl Limit {
    pub fn new(limit_price: MinNonNan, total_volume: MinNonNan) -> Self {
        Limit {
            limit_price: limit_price,
            total_volume: total_volume
        }
    }
}

impl Book {
    pub fn new() -> Self {
        Book {
            buy_tree: BTreeMap::new(),
            sell_tree: BTreeMap::new(),
        }
    }
}
