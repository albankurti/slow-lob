use std::collections::BTreeMap;
use serde_json::json;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use websocket::{client::builder::ClientBuilder, message::OwnedMessage};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use crate::venues::venue_traits::VenueFunctionality;
use crate::orderbook::{Limit, Book};
use std::cmp::Reverse;
use crate::utils::{from_float, print};
use ordered_float::NotNan;

type MinNonNan = Reverse<NotNan<f64>>;

pub struct BinanceBook{
    pub name: String,
    pub base_ws: String,
    pub symbol: String,
    pub method: String,
    pub limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct BinanceResponse{
    id: String,
    status: i32,
    result: Result
}

#[derive(Debug, Deserialize, Serialize)]
struct Result{
    lastUpdateId: i64,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}


impl VenueFunctionality for BinanceBook{
    fn subscribe(&self, buy_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>, sell_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>) {
        let mut websocket = ClientBuilder::new(&self.base_ws)
        .unwrap()
        .connect(None)
        .expect("Failed to connect to websocket");

        let mut rng = thread_rng();
        let random_id: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();

        let message = json!({
            "id": random_id,
            "method": &self.method,
            "params": {
                "symbol": &self.symbol,
                "limit": &self.limit
            },
        });

        websocket.send_message(&OwnedMessage::Text(message.to_string()))
            .expect("Failed to send message");

        loop {
            let message = match websocket.recv_message() {
                Ok(message) => message,
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                    break;
                }
            };

            match message {
                OwnedMessage::Text(s) => {
                    self.feed_orderbook(s, buy_tree, sell_tree);
                },
                OwnedMessage::Binary(b) => println!("Binary message received: {:?}", b),
                OwnedMessage::Close(_) => {
                    println!("Closing connection");
                    break;
                }
                _ => println!("Other message received"),
            }
        }
    }

    fn unsubscribe(&self) {
       // TODO
    }

    // Needs to parse a Text file and create a
    fn feed_orderbook(&self, data: String, buy_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>, sell_tree: &mut Arc<Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>) {
        match serde_json::from_str::<BinanceResponse>(&data){
            Ok(res) => {
                for ask in res.result.asks{
                    let price = from_float(ask[0].parse().unwrap());
                    let volume = vec![(from_float(ask[1].parse().unwrap()), self.name.clone())];
                    let limit = Limit::new(price, volume.clone(), volume[0].0, chrono::Local::now());
                    Book::check_insert(limit, &mut sell_tree.lock().unwrap());
                }
                for bid in res.result.bids{
                    let price = from_float(bid[0].parse().unwrap());
                    let volume = vec![(from_float(bid[1].parse().unwrap()), self.name.clone())];
                    let limit = Limit::new(price, volume.clone(), volume[0].0, chrono::Local::now());
                    Book::check_insert(limit, &mut buy_tree.lock().unwrap());
                }
            }
            Err(e) => {
                print("Could not parse incoming data");
                print(e);
            }
        }
    }
}











