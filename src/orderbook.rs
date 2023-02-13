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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Limit {
   pub limit_price: f64,
   pub total_volume: f64,
   pub left_child: Option<Box<Limit>>,
   pub right_child: Option<Box<Limit>>,
}

impl Book {
    pub fn print_in_order(&self, tree: &Option<Box<Limit>>){
        if let Some(node) = tree {
            self.print_in_order(&node.left_child);
            println!("{:?}, {:?}\n", node.limit_price, node.total_volume);
            self.print_in_order(&node.right_child);
        }
    }

    pub fn update_limit_node(&mut self, limit_price: f64, size: f64, side: bool) {
        // this function will update a limit node if it exists, or create a new one if it doesn't exist
        let mut current_node: &mut Option<Box<Limit>>;
        if side {
            current_node = &mut self.buy_tree;
        } else {
            current_node = &mut self.sell_tree;
        }
        let mut update_minmax = |check_node: &Box<Limit>| {
            if side {
                match &mut self.highest_buy {
                    None => {
                        self.highest_buy = Some(check_node.clone());
                    },
                    Some(highest) => {
                        if check_node.limit_price > highest.limit_price {
                            self.highest_buy = Some(check_node.clone());
                        }
                    }
                }
            } else {
                match &mut self.lowest_sell {
                    None => {
                        self.lowest_sell = Some(check_node.clone());
                    },
                    Some(lowest) => {
                        if check_node.limit_price < lowest.limit_price {
                            self.lowest_sell = Some(check_node.clone());
                        }
                    }
                }
            }
        };
        let new_node = Box::new(Limit{
            limit_price,
            total_volume: size,
            left_child: None,
            right_child: None,
        });
        loop {
            match current_node{
                None => {
                        // if tree is not initialized
                        current_node.replace(new_node);
                        update_minmax(current_node.as_ref().expect("Node must exist."));
                        break;
                },
                Some(node) => {
                    if limit_price > node.limit_price {
                        if node.right_child.is_some() {
                            current_node = &mut node.right_child;
                        } else {
                            node.right_child = Some(new_node);
                            update_minmax(node);
                            break;
                        }
                    } else if limit_price < node.limit_price {
                        if node.left_child.is_some() {
                            current_node = &mut node.left_child;
                        }
                        else{
                            node.left_child = Some(new_node);
                            update_minmax(node);
                            break;
                        }
                    } else {
                        node.total_volume = size;
                        break;
                    }
                }
            }
        }
    }

    pub fn print_minmax(&self){
        print!("Min: {:?}, Max: {:?}", self.lowest_sell, self.highest_buy);
    }

    fn cancel_limit(&mut self, limit_price: f64, side: bool) {
        todo!();
    }

    fn get_best_bid(&self) -> i32 {
        todo!();
    }

    fn get_best_offer(&self) -> i32 {
        todo!();
    }

}