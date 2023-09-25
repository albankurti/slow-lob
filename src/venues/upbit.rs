use std::collections::BTreeMap;
use std::process::exit;
use std::{time,env};
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
use serde_json::json;
use serde::{Deserialize, Serialize};
use websocket::{client::builder::ClientBuilder, message::OwnedMessage, result::WebSocketError};
use rand::{Rng, thread_rng, random};
use rand::distributions::Alphanumeric;
use crate::venues::venue_traits::VenueFunctionality;
use crate::orderbook::Limit;
use std::cmp::Reverse;
use ordered_float::NotNan;
type MinNonNan = Reverse<NotNan<f64>>;

pub struct UpbitBook {
    pub name: String,
    pub base_ws: String,
    pub channel: String,
    pub codes: Vec<String>,
}

pub struct UpbitResponse{

}

impl VenueFunctionality for UpbitBook{
    fn subscribe(&self, buy_tree: &mut BTreeMap<MinNonNan, Limit>, sell_tree: &mut BTreeMap<MinNonNan, Limit>) {
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

    fn feed_orderbook(&self, data: String, buy_tree: &mut BTreeMap<MinNonNan, Limit>, sell_tree: &mut BTreeMap<MinNonNan, Limit>) {
        todo!();
    }
}