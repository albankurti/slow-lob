use std::collections::BTreeMap;
use serde_json::json;
use std::sync::{Arc};
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use websocket::{client::builder::ClientBuilder, message::OwnedMessage};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use crate::orderbook::{Limit, Book, WrappedReverse};
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


impl BinanceBook{
    pub async fn subscribe(&self, bids: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>, asks: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>) {
        let mut websocket = ClientBuilder::new(&self.base_ws)
        .unwrap()
        .connect(None)
        .expect("Failed to connect to websocket");

        let random_id: String = (0..16).map(|_| thread_rng().sample(Alphanumeric) as char).collect();

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
                    self.feed_orderbook(s, bids, asks).await;
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
    async fn feed_orderbook(&self, data: String, bids: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>, asks: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>) {
        match serde_json::from_str::<BinanceResponse>(&data){
            Ok(res) => {
                for ask in res.result.asks{
                    let price = from_float(ask[0].parse().unwrap());
                    let volume = vec![(from_float(ask[1].parse().unwrap()), self.name.clone())];
                    let limit = Limit::new(price, volume.clone(), volume[0].0, chrono::Local::now());
                    Book::check_insert(limit, asks).await;
                }
                for bid in res.result.bids{
                    let price = from_float(bid[0].parse().unwrap());
                    let volume = vec![(from_float(bid[1].parse().unwrap()), self.name.clone())];
                    let limit = Limit::new(price, volume.clone(), volume[0].0, chrono::Local::now());
                    Book::check_insert(limit, bids).await;
                }
            }
            Err(e) => {
                print("Could not parse incoming data");
                print(e);
                print(data);
            }
        }
    }
}











