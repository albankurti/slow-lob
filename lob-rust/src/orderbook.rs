use std::{cmp::Reverse, fmt};
use std::sync::Arc;
use std::ops::Deref;
use ordered_float::NotNan;

use std::collections::BTreeMap;
use chrono::Local;

use crate::utils::{from_float, print};

type MinNonNan = Reverse<NotNan<f64>>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct WrappedReverse(pub Reverse<NotNan<f64>>);

#[derive(Debug)]
pub struct Book {
    pub bids: BTreeMap<WrappedReverse, Limit>,
    pub asks: BTreeMap<WrappedReverse, Limit>
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
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    /*
    This function will see if the node exists. If it does not, we insert.
    If it does exist, it will check if the volume origin exists, if yes,
    then replace with new value, if not then we just push the volume and 
    origin to the existing node's volumes
    And then in each case we update the total_volume as well
    */
    pub async fn check_insert(limit: Limit, tree: &mut Arc<tokio::sync::Mutex<BTreeMap<WrappedReverse, Limit>>>){
        let mut tree = tree.lock().await;
        let mut found = false;
        let mut is_zero = false;
        let limit_volume = *limit.total_volume.0.0;
        if let Some(node) = tree.get_mut(&limit.limit_price.clone()){
            for i in 0..node.volumes.len(){
                // if we find a node and the same volume origin as the incoming limit
                if node.volumes[i].1 == limit.volumes[0].1 {
                    // we update the total_volume by subtracting current volume and adding the updated volume
                    // the first .0 is to select the first element of tuple, the second .0 is to select the
                    // f64 from the WrappedReverse
                    found = true;
                    node.total_volume = WrappedReverse(Reverse(node.total_volume.0.0 - node.volumes[i].0.0.0 + limit.volumes[0].0.0.0));
                    node.volumes[i].0 = limit.volumes[0].0;
                    if node.total_volume == from_float(0.0) {
                        is_zero = true;
                    }
                    break;
                }
            } if !found {
                // if we find the node but volume origin does not exist
                node.volumes.push(limit.volumes[0].clone());
                // new origin comes so just add to the current total_volume
                node.total_volume = WrappedReverse(Reverse(node.total_volume.0.0 + limit.volumes[0].0.0.0));
                if node.total_volume == from_float(0.0){
                    is_zero = true;
                }
            }
        } else if limit_volume > 0.0 {
            // if we never found a node, this also has a total_volume equal to the origin volume
            // in the other cases, the total_volume of the incoming limit is ignored
            tree.insert(limit.limit_price, limit.clone());
        }
        if is_zero {
            tree.remove(&limit.limit_price);
        }    
    }

}
