use std::time;
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
use serde_json::json;
use websocket::{client::builder::ClientBuilder, message::OwnedMessage, result::WebSocketError};

trait ExchangeFunc {
    fn generate_hmac(&self) -> String;
    fn subscribe(&self);
    fn unsubscribe(&self);
    fn parse_data(&self);
}

struct Coinbase {
    name: String,
    base_ws: String,
    api_key: String,
    secret: String,
    product_ids: Vec<String>,
    channel: String,
}

impl ExchangeFunc for Coinbase{
    fn generate_hmac(&self) -> String {
        let timestamp = (time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i64).to_string();
        let product_ids_string = &self.product_ids.join(",");
        let prehash: String = format!("{}{}{}", timestamp, &self.channel, product_ids_string);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(&self.secret.as_bytes()).unwrap();
        mac.update(prehash.as_bytes());
        let code_bytes = mac.finalize().into_bytes();

        return hex::encode(&code_bytes.to_vec());
    }

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
