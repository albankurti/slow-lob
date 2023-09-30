use std::collections::BTreeMap;
use std::cmp::Reverse;
use ordered_float::NotNan;
use std::sync::{Arc, Mutex};
use crate::orderbook::Limit;
type MinNonNan = Reverse<NotNan<f64>>;

pub trait VenueFunctionality {
    fn subscribe(&self, buy_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>, sell_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>);
    fn unsubscribe(&self);
    fn feed_orderbook(&self, data: String, buy_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>, 
        sell_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>);
}