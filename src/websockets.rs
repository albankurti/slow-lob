use std::{time,env};
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
use serde_json::json;
use websocket::{client::builder::ClientBuilder, message::OwnedMessage, result::WebSocketError};
use rand::{Rng, thread_rng, random};
use rand::distributions::Alphanumeric;
use crate::exchange_traits::ExchangeFunctionality;

pub struct CoinbaseBook {
    pub name: String,
    pub base_ws: String,
    pub api_key: String,
    pub secret: String,
    pub product_ids: Vec<String>,
    pub channel: String,
}

impl CoinbaseBook{
    pub fn generate_hmac(&self) -> String {
        let timestamp = (time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i64).to_string();
        let product_ids_string = &self.product_ids.join(",");
        let prehash: String = format!("{}{}{}", timestamp, &self.channel, product_ids_string);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(&self.secret.as_bytes()).unwrap();
        mac.update(prehash.as_bytes());
        let code_bytes = mac.finalize().into_bytes();

        return hex::encode(&code_bytes.to_vec());
    }

    pub fn get_api_key() -> String {
        env::var("API_KEY").expect("API_KEY environment variable is not set")
    }

    pub fn get_api_secret() -> String {
        env::var("API_SECRET").expect("API_SECRET environment variable is not set")
    }
}

impl ExchangeFunctionality for CoinbaseBook{
    fn subscribe(&self){
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
                OwnedMessage::Text(s) => println!("Text message received: {}", s),
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

    fn parse_data(&self) {
        //TODO
    }
}

pub struct BinanceBook{
    pub name: String,
    pub base_ws: String,
    pub symbol: String,
    pub method: String,
    pub limit: u32,
}

impl ExchangeFunctionality for BinanceBook{
    fn subscribe(&self) {
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
                OwnedMessage::Text(s) => println!("Text message received: {}", s),
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

    }

    fn parse_data(&self) {

    }
}

pub struct UpbitBook {
    pub name: String,
    pub base_ws: String,
    pub channel: String,
    pub codes: Vec<String>,
}

impl ExchangeFunctionality for UpbitBook{
    fn subscribe(&self) {
        let mut websocket = ClientBuilder::new(&self.base_ws)
        .unwrap()
        .connect(None)
        .expect("Failed to connect to websocket");

        let mut rng = thread_rng();
        let random_id: String = (0..6).map(|_| rng.sample(Alphanumeric) as char).collect();

        let message = json!(
        [{
                    "ticket": random_id,
                }, {
                    "type": &self.channel,
                    "codes": &self.codes,
                    "isOnlyRealtime": true,
                }]
        );

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
                OwnedMessage::Text(s) => println!("Text message received: {}", s),
                OwnedMessage::Binary(b) => println!("Binary message received: {:?} \n From binary to string: {:?}", b, String::from_utf8(b.clone())),
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

    fn parse_data(&self) {
        // TODO
    }
}
















