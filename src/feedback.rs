use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use ordered_float::NotNan;
use crate::orderbook::{Limit};
use std::{cmp::Reverse};
use std::collections::BTreeMap;
use crate::orderbook::WrappedReverse;
use serde_json;
use futures::SinkExt;
use futures::StreamExt;
use std::sync::Arc;

pub async fn feedback(
    tree: Arc<std::sync::Mutex<BTreeMap<WrappedReverse, Limit>>>,
    port: u16) {

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    println!("WebSocket server is running on ws://127.0.0.1:{}", port);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        println!("WebSocket connection established");

        let (mut sender, _) = ws_stream.split();

        loop {
            {
                let data = serde_json::to_string(&format!("{:?}", tree.lock().unwrap())).unwrap();
                let message = Message::Text(data);
                sender.send(message).await.unwrap();
            }
        }
    }
}