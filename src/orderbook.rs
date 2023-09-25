use std::{cmp::Reverse, time::Instant};
use std::ops::Add;
use ordered_float::NotNan;
use std::collections::BTreeMap;
use chrono::{Local, DateTime, Timelike};

use crate::utils::from_float;

type MinNonNan = Reverse<NotNan<f64>>;

#[derive(Debug)]
pub struct Book {
    pub buy_tree: BTreeMap<MinNonNan, Limit>,
    pub sell_tree: BTreeMap<MinNonNan, Limit>
}

#[derive(Debug, Clone)]
pub struct Limit {
    pub limit_price: MinNonNan,
    pub total_volume: MinNonNan,
    pub volumes: Vec<(MinNonNan, String)>,
    pub last_update: chrono::DateTime<Local>
}

impl Limit {
    pub fn new(limit_price: MinNonNan, volumes: Vec<(MinNonNan, String)>, total_volume: MinNonNan, last_update: chrono::DateTime<Local>) -> Self {
        Limit {
            limit_price: limit_price,
            volumes: volumes,
            total_volume: total_volume,
            last_update: last_update
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

    /*
    This function will see if the node exists. If it does not, we insert.
    If it does exist, it will check if the volume origin exists, if yes,
    then replace with new value, if not then we just push the volume and 
    origin to the existing node's volumes
    And then in each case we update the total_volume as well
    */
    pub fn check_insert(self, limit: &mut Limit, tree: &mut BTreeMap<MinNonNan, Limit>){
        if let Some(node) = tree.get_mut(&limit.limit_price.clone()){
            for i in 0..node.volumes.len(){
                // if we find a node and the same volume origin as the incoming limit
                if node.volumes[i].1 == limit.volumes[0].1 {
                    // we update the total_volume by subtracting current volume and adding the updated volume
                    // the first .0 is to select the first element of tuple, the second .0 is to select the
                    // f64 from the MinNonNan
                    node.total_volume = Reverse(node.total_volume.0 - node.volumes[i].0.0 + limit.volumes[0].0.0);
                    node.volumes[i].0 = limit.volumes[0].0;
                    return;
                }
            }
            // if we find the node but volume origin does not exist
            node.volumes.push(limit.volumes[0].clone());
            // new origin comes so just add to the current total_volume
            node.total_volume = Reverse(node.total_volume.0 + limit.volumes[0].0.0);
        } else {
            // if we never found a node, this also has a total_volume equal to the origin volume
            // in the other cases, the total_volume of the incoming limit is ignored
            tree.insert(limit.limit_price, limit.clone());
        }
    }
}
