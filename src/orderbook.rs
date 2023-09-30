use std::{cmp::Reverse, fmt};
use std::ops::Deref;
use ordered_float::NotNan;

use std::collections::BTreeMap;
use chrono::Local;

use crate::utils::{from_float};

type MinNonNan = Reverse<NotNan<f64>>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedReverse(pub Reverse<NotNan<f64>>);

#[derive(Debug)]
pub struct Book {
    pub buy_tree: BTreeMap<WrappedReverse, Limit>,
    pub sell_tree: BTreeMap<WrappedReverse, Limit>
}

#[derive(Clone)]
pub struct Limit {
    pub limit_price: WrappedReverse,
    pub total_volume: WrappedReverse,
    pub volumes: Vec<(WrappedReverse, String)>,
    pub last_update: chrono::DateTime<Local>
}

impl Limit {
    pub fn new(limit_price: WrappedReverse, volumes: Vec<(WrappedReverse, String)>, total_volume: WrappedReverse, last_update: chrono::DateTime<Local>) -> Self {
        Limit {
            limit_price,
            volumes,
            total_volume,
            last_update
        }
    }
}

impl fmt::Debug for Limit{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Limit {{ limit_price: {:?}, total_volume: {:?}, volumes: [",
            *self.limit_price.0.0,
            *self.total_volume.0.0,
        )?;
        
        for (volume, origin) in &self.volumes {
            write!(f, "({}, \"{}\"), ", *volume.0.0, origin)?;
        }
        
        write!(f, "], last_update: {:?} }}", self.last_update)
    }
}

impl fmt::Debug for WrappedReverse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", *self.0.0)
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
    pub fn check_insert(limit: Limit, tree: &mut BTreeMap<WrappedReverse, Limit>){
        if let Some(node) = tree.get_mut(&limit.limit_price.clone()){
            for i in 0..node.volumes.len(){
                // if we find a node and the same volume origin as the incoming limit
                if node.volumes[i].1 == limit.volumes[0].1 {
                    // we update the total_volume by subtracting current volume and adding the updated volume
                    // the first .0 is to select the first element of tuple, the second .0 is to select the
                    // f64 from the WrappedReverse
                    node.total_volume = WrappedReverse(Reverse(node.total_volume.0.0 - node.volumes[i].0.0.0 + limit.volumes[0].0.0.0));
                    node.volumes[i].0 = limit.volumes[0].0;
                    return;
                }
            }
            // if we find the node but volume origin does not exist
            node.volumes.push(limit.volumes[0].clone());
            // new origin comes so just add to the current total_volume
            node.total_volume = WrappedReverse(Reverse(node.total_volume.0.0 + limit.volumes[0].0.0.0));
        } else {
            // if we never found a node, this also has a total_volume equal to the origin volume
            // in the other cases, the total_volume of the incoming limit is ignored
            tree.insert(limit.limit_price, limit.clone());
        }
        if let Some(node) = tree.get(&limit.limit_price){
            if node.total_volume == from_float(0.0){
                tree.remove(&limit.limit_price);
            }
        }
    }

}
