use std::collections::VecDeque;
use std::thread::current;

/*
Preliminary Orderbook data structure implementation
Intended to create an AVL Tree soon
*/
#[derive(Debug)]
pub struct Book {
    pub buy_tree: Option<Box<Limit>>,
    pub sell_tree: Option<Box<Limit>>,
    pub lowest_sell: Option<Box<Limit>>,
    pub highest_buy: Option<Box<Limit>>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Limit {
    pub limit_price: f64,
    pub total_volume: f64,
    pub left_child: Option<Box<Limit>>,
    pub right_child: Option<Box<Limit>>,
}

impl Limit {
    pub fn new(limit_price: f64, total_volume: f64) -> Self {
        Limit {
            limit_price: limit_price,
            total_volume: total_volume,
            left_child: None,
            right_child: None,
        }
    }

    pub fn left(mut self, node: Limit) -> Self {
        self.left_child = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Limit) -> Self {
        self.right_child = Some(Box::new(node));
        self
    }
}

impl Book {
    pub fn insert(&mut self, limit_price: f64, total_volume: f64, side: bool) {
        let mut current = if side {
            &mut self.buy_tree
        } else {
            &mut self.sell_tree
        };
        while let Some(ref mut node) = *current {
            if node.limit_price < limit_price {
                current = &mut node.left_child;
            } else if node.limit_price > limit_price {
                current = &mut node.right_child;
            } else {
                node.total_volume += total_volume;
                return;
            }
        }
        *current = Some(Box::new(Limit::new(limit_price, total_volume)));

        // updating min and max of book
        if side {
            if let Some(ref mut node) = self.highest_buy {
                if limit_price > node.limit_price {
                    self.highest_buy = current.clone();
                }
            }
        } else {
            if let Some(ref mut node) = self.lowest_sell {
                if limit_price < node.limit_price {
                    self.lowest_sell = current.clone();
                }
            }
        }
    }
    // pub fn remove(&mut self, limit_price: f64, side: bool) -> bool {
    //     let mut current = if side {
    //         &mut self.buy_tree
    //     } else {
    //         &mut self.sell_tree
    //     };
    //     while let Some(ref mut node) = current {
    //         if limit_price > node.limit_price {
    //             current = &mut node.right_child;
    //         } else if limit_price < node.limit_price {
    //             current = &mut node.left_child;
    //         } else {
    //             return true;
    //         }
    //     }

    //     // if no node was found with the limit_price
    //     false
    // }
}
