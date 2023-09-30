use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::handshake::server::Request;
use tokio_tungstenite::accept_async;
use ordered_float::NotNan;
use crate::orderbook::{Limit, Book};
use std::{cmp::Reverse};
use std::collections::BTreeMap;
use tokio_tungstenite::WebSocketStream;
use futures::SinkExt;
use futures::StreamExt;
use crate::utils::{print};
use std::sync::{Arc, Mutex, MutexGuard};

pub async fn feedback(
    buy_tree: Arc<std::sync::Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>,
    sell_tree: Arc<std::sync::Mutex<BTreeMap<Reverse<NotNan<f64>>, Limit>>>) {

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("WebSocket server is running on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        println!("WebSocket connection established");

        let (mut sender, _) = ws_stream.split();

        loop {
            {
                let data = format!("{:?}", buy_tree.lock().unwrap());
                let message = Message::Text(data);
                sender.send(message).await.unwrap();
            }
        }
    }
}