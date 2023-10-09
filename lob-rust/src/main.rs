mod orderbook;
mod utils;
mod venues;
mod feedback;
use feedback::{feedback};

use tokio::join;
use orderbook::Book;
use utils::{read_keys};
use std::sync::Arc;
use tokio::sync::Mutex;

use venues::binance::BinanceBook;
use venues::coinbase::CoinbaseBook;

#[tokio::main]
async fn main() {
    read_keys("keys.txt".to_string());

    let binance: BinanceBook = BinanceBook {
       name: String::from("binance"),
       base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
       symbol: String::from("ETHUSDT"),
       method: String::from("depth"),
       limit: 50,
    };

    let coinbase: CoinbaseBook = CoinbaseBook {
       name: String::from("coinbase"),
       base_ws: String::from("wss://advanced-trade-ws.coinbase.com"),
       api_key: CoinbaseBook::get_api_key(),
       secret: CoinbaseBook::get_api_secret(),
       product_ids: vec![String::from("ETH-USD")],
       channel: String::from("level2"),
    };

    let bids_original = Arc::new(Mutex::new(Book::new().bids));
    let asks_original = Arc::new(Mutex::new(Book::new().asks));

    // let mut bids = Arc::clone(&bids_original);
    // let mut asks = Arc::clone(&asks_original);
    // let handle_binance = tokio::spawn(async move {
    //     binance.subscribe(&mut bids, &mut asks).await;
    // });

    let mut bids = Arc::clone(&bids_original);
    let mut asks = Arc::clone(&asks_original);
    let handle_coinbase = tokio::spawn(async move {
        coinbase.subscribe(&mut bids, &mut asks).await;
    });

    let bids = Arc::clone(&bids_original);
    let asks = Arc::clone(&asks_original);
    let handle_stream = tokio::spawn(async move {
        feedback(bids, asks).await;
    });

    _ = join!(handle_coinbase, handle_stream);

}