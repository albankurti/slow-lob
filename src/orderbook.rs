use std::collections::VecDeque;
use std::mem;
use std::thread::current;

/*
Preliminary Orderbook data structure implementation
Intended to create an AVL Tree soon
*/
#[derive(Debug)]
pub struct Book {
    // root of buy side
    pub buy_tree: Option<Box<Limit>>,
    // root of sell side
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

    pub fn print_tree(node: &Option<Box<Limit>>, prefix: &str, is_left: bool) {
        if let Some(node) = node {
            let _ = Self::print_tree(
                &node.right_child,
                &(prefix.to_string() + (if is_left { "│   " } else { "    " })),
                false,
            );

            println!(
                "{}{}{}{}{}, {}",
                prefix,
                if is_left { "└── " } else { "┌── " },
                "",
                node.limit_price,
                "",
                node.total_volume
            );

            let _ = Self::print_tree(
                &node.left_child,
                &(prefix.to_string() + (if is_left { "    " } else { "│   " })),
                true,
            );
        }
    }

    pub fn traverse(root: Option<&Box<Limit>>) {
        if let Some(node) = root {
            Limit::traverse(node.left_child.as_ref());
            println!("{}", &node.limit_price);
            Limit::traverse(node.right_child.as_ref());
        }
    }

    pub fn remove(node: &mut Option<Box<Limit>>, limit_price: f64) {
        if let Some(ref mut n) = node {
            if limit_price < n.limit_price {
                Self::remove(&mut n.left_child, limit_price);
            } else if limit_price > n.limit_price {
                Self::remove(&mut n.right_child, limit_price);
            } else {
                println!("");
                *node = match (n.left_child.take(), n.right_child.take()) {
                    (None, None) => {
                        println!("Replacing {:?} with None", n.limit_price);
                        None
                    }
                    (Some(left), None) => {
                        println!("Replacing {:?} with: {:?}", n.limit_price, left.limit_price);
                        Some(left)
                    }
                    (None, Some(right)) => {
                        println!(
                            "Replacing {:?} with: {:?}",
                            n.limit_price, right.limit_price
                        );
                        Some(right)
                    }
                    (Some(left), Some(right)) => {
                        n.left_child = Some(left);
                        let mut succ: &Box<Limit> = &right;
                        while let Some(ref succ_right) = succ.left_child {
                            succ = &succ_right;
                        }
                        println!("Replacing {:?} with: {:?}", n.limit_price, succ.limit_price);
                        // Copy the successor's data to the current node
                        n.limit_price = succ.limit_price;
                        n.total_volume = succ.total_volume;
                        n.right_child = Some(right);
                        // Recursively remove the successor
                        Self::remove(&mut n.right_child, n.limit_price);
                        Self::print_tree(&Some(n.clone()), "", false);
                        // The current node still remains
                        Some(n.clone())
                    }
                };
            }
        }
        println!("");
        Self::print_tree(node, "", false);
        println!("");
    }

    pub fn left(mut self, node: Limit) -> Self {
        self.left_child = Some(Box::new(node));
        self
    }

    pub fn right(mut self) -> Option<Box<Limit>> {
        self.right_child
    }

    pub fn rotate_left(node: &mut Box<Limit>) {
        let mut new_root = node.right_child.take().unwrap();
        node.right_child = new_root.left_child.take();
        new_root.left_child = Some(node.clone());
        *node = new_root;
    }

    fn rotate_right(node: &mut Box<Limit>) {
        let mut new_root = node.left_child.take().unwrap();
        node.left_child = new_root.right_child.take();
        new_root.right_child = Some(node.clone());
        *node = new_root;
    }

    pub fn get_balance(node: &Box<Limit>) -> i32 {
        Limit::get_height(&node.left_child) - Limit::get_height(&node.right_child)
    }

    pub fn get_height(node: &Option<Box<Limit>>) -> i32 {
        match node {
            None => 0,
            Some(inner) => {
                1 + std::cmp::max(
                    Limit::get_height(&inner.left_child),
                    Limit::get_height(&inner.right_child),
                )
            }
        }
    }

    pub fn balance(node: &mut Box<Limit>) {
        if Limit::get_balance(node) > 1 {
            // Left heavy
            if Limit::get_balance(&mut node.left_child.as_mut().unwrap()) < 0 {
                // Left-Right case
                Limit::rotate_left(&mut node.left_child.as_mut().unwrap());
            }
            Limit::rotate_right(node);
        } else if Limit::get_balance(node) < -1 {
            // Right heavy
            if Limit::get_balance(&mut node.right_child.as_mut().unwrap()) > 0 {
                // Right-Left case
                Limit::rotate_right(&mut node.right_child.as_mut().unwrap());
            }
            Limit::rotate_left(node);
        }
    }

    pub fn get_min(node: Box<Limit>) -> f64 {
        // TODO: delete/drop the node as well before returning the limit_price
        // initializing the search on the right child of the node
        let mut res: Option<Box<Limit>> = node.right_child;
        while let Some(curr) = res {
            if let Some(left) = curr.left_child {
                res = Some(left);
            } else {
                return curr.limit_price;
            }
        }
        res.unwrap().limit_price
    }
}

impl Book {
    pub fn new() -> Self {
        Book {
            buy_tree: None,
            sell_tree: None,
            lowest_sell: None,
            highest_buy: None,
        }
    }

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
}
