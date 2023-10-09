use chrono::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use ordered_float::NotNan;
use warp::{Filter};
use http;
use crate::orderbook::{Limit};
use std::{cmp::Reverse};
use std::collections::BTreeMap;
use crate::orderbook::WrappedReverse;
use crate::utils::print;
use serde_json;
use futures::SinkExt;
use futures::StreamExt;
use std::sync::{Arc};
use tokio::sync::Mutex;
use rand::Rng;

pub async fn feedback(
    bids: Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>,
    asks: Arc<Mutex<BTreeMap<WrappedReverse, Limit>>>) {

    let ws_filter = warp::path("websocket")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let bids = Arc::clone(&bids);
            let asks = Arc::clone(&asks);
            ws.on_upgrade(move |websocket| async move {
                let (mut sender, _) = websocket.split();
                loop {
                    {
                        let asks = asks.lock().await;
                        let ask_sequence: Vec<_> = asks.iter().rev().take(10).collect();

                        let bids = bids.lock().await;
                        let bid_sequence: Vec<_> = bids.iter().take(10).collect();

                        let data = serde_json::to_string(&format!("{:?}, {:?}", ask_sequence, bid_sequence)).unwrap();
                        let message = warp::ws::Message::text(data);

                        sender.send(message).await.unwrap();
                    }
                }   

            })
        });

    // Combine the WebSocket filter and CORS middleware
    let routes = ws_filter
        .with(warp::cors()
        .allow_any_origin()
        .allow_methods([http::Method::GET, http::Method::POST]));

    // Start the WebSocket server with CORS support
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8085))
        .await;
}
