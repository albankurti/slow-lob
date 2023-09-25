use std::collections::BTreeMap;
use std::cmp::Reverse;
use ordered_float::NotNan;
use crate::orderbook::Limit;
type MinNonNan = Reverse<NotNan<f64>>;

pub trait VenueFunctionality {
    fn subscribe(&self, buy_tree: &mut BTreeMap<MinNonNan, Limit>, sell_tree: &mut BTreeMap<MinNonNan, Limit>);
    fn unsubscribe(&self);
    fn feed_orderbook(&self, data: String, buy_tree: &mut BTreeMap<MinNonNan, Limit>, sell_tree: &mut BTreeMap<MinNonNan, Limit>);
}