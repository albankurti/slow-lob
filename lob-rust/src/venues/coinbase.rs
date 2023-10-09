use std::collections::BTreeMap;
use std::{time,env};
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use std::sync::{Arc};
use tokio::sync::Mutex;
use serde_json::{json};
use serde::{Deserialize, Serialize};
use websocket::{client::builder::ClientBuilder, message::OwnedMessage};
use crate::utils::{from_float, print};
use crate::orderbook::{Limit, Book, WrappedReverse};
use std::cmp::Reverse;
use ordered_float::NotNan;

type MinNonNan = Reverse<NotNan<f64>>;

pub struct CoinbaseBook {
    pub name: String,
    pub base_ws: String,
    pub api_key: String,
    pub secret: String,
    pub product_ids: Vec<String>,
    pub channel: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CoinbaseResponse{
    channel: String,
    client_id: String,
    timestamp: String,
    sequence_num: i32,
    events: Vec<Event>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Event {
    r#type: String,
    product_id: String,
    updates: Vec<Update>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Update {
    side: String,
    event_time: String,
    price_level: String,
    new_quantity: String,
}

impl CoinbaseBook{
    pub fn generate_hmac(&self) -> String {
        let timestamp = (time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i64).to_string();
        let product_ids_string = &self.product_ids.join(",");
        let prehash: String = format!("{}{}{}", timestamp, &self.channel, product_ids_string);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        mac.update(prehash.as_bytes());
        let code_bytes = mac.finalize().into_bytes();

        hex::encode(code_bytes)
    }

    pub fn get_api_key() -> String {
        env::var("API_KEY").expect("API_KEY environment variable is not set")
    }

    pub fn get_api_secret() -> String {
        env::var("API_SECRET").expect("API_SECRET environment variable is not set")
    }

    pub async fn subscribe(&self, bids: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>, asks: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>){
        let mut websocket = ClientBuilder::new(&self.base_ws)
        .unwrap()
        .connect(None)
        .expect("Failed to connect to websocket");

        let timestamp = (time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i64).to_string();
        let signature: String = self.generate_hmac();

        let message = json!({
            "type": "subscribe",
            "product_ids": &self.product_ids,
            "channel": &self.channel,
            "api_key": &self.api_key,
            "timestamp": &timestamp,
            "signature": &signature,
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
                }
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
        //TODO
    }

    async fn feed_orderbook(&self, data: String, bids: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>, asks: &mut Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>) {
        match serde_json::from_str::<CoinbaseResponse>(&data) {
            Ok(res) => {
                for update in res.events[0].updates.iter(){
                    let price = from_float(update.price_level.parse().unwrap());
                    let volume = vec![(from_float(update.new_quantity.parse().unwrap()), self.name.clone())];
                    let limit = Limit::new(price, volume.clone(), volume[0].0, chrono::Local::now());
                    match update.side.as_str() {
                        "ask" | "offer" => {
                            Book::check_insert(limit, asks).await;
                        },
                        "bid" => {
                            Book::check_insert(limit, bids).await;
                        },
                        _ => println!("Could not determine whether the type of the order: {:?}\n", update.side.as_str())
                    }
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
