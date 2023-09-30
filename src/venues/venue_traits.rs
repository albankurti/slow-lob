use std::collections::BTreeMap;
use std::cmp::Reverse;
use ordered_float::NotNan;
use std::sync::{Arc, Mutex};
use crate::orderbook::{Limit, WrappedReverse};

pub trait VenueFunctionality {
    fn subscribe(&self, buy_tree: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>, sell_tree: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>);
    fn unsubscribe(&self);
    fn feed_orderbook(&self, data: String, buy_tree: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>, 
        sell_tree: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>);
}