mod orderbook;
mod utils;
mod venues;
mod feedback;
use feedback::{feedback};
use tokio::net::TcpListener;
use tokio::join;
use orderbook::Book;
use utils::{print, export_txt, read_keys};
use std::sync::{Arc, Mutex};
use std::env;
use venues::binance::BinanceBook;
use venues::coinbase::CoinbaseBook;
use venues::upbit::UpbitBook;
use venues::venue_traits::VenueFunctionality;

#[tokio::main]
async fn main() {
    read_keys("keys.txt".to_string());

    let binance: BinanceBook = BinanceBook {
       name: String::from("binance"),
       base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
       symbol: String::from("BTCUSDT"),
       method: String::from("depth"),
       limit: 50,
    };

    let coinbase: CoinbaseBook = CoinbaseBook {
       name: String::from("coinbase"),
       base_ws: String::from("wss://advanced-trade-ws.coinbase.com"),
       api_key: CoinbaseBook::get_api_key(),
       secret: CoinbaseBook::get_api_secret(),
       product_ids: vec![String::from("BTC-USD")],
       channel: String::from("level2"),
    };

    let buy_tree_original = Arc::new(Mutex::new(Book::new().buy_tree));
    let sell_tree_original = Arc::new(Mutex::new(Book::new().sell_tree));

    let mut buy_tree = Arc::clone(&buy_tree_original);
    let mut sell_tree = Arc::clone(&sell_tree_original);
    let handle_binance = tokio::spawn(async move {
        binance.subscribe(&mut buy_tree, &mut sell_tree);
    });

    let mut buy_tree = Arc::clone(&buy_tree_original);
    let mut sell_tree = Arc::clone(&sell_tree_original);
    let handle_coinbase = tokio::spawn(async move {
        coinbase.subscribe(&mut buy_tree, &mut sell_tree);
    });

    let buy_tree = Arc::clone(&buy_tree_original);
    let sell_tree = Arc::clone(&sell_tree_original);
    let handle_stream = tokio::spawn(async move {
        feedback(buy_tree, sell_tree).await;
    });

    _ = join!(handle_binance, handle_coinbase, handle_stream);
// 
}