mod orderbook;
mod utils;
mod venues;
use digest::typenum::Or;
use orderbook::{Book, Limit};
use std::env;
use bst_rs::{BinarySearchTree, IterativeBST, RecursiveBST};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::cmp::Reverse;
use ordered_float::NotNan;
use utils::{to_float, print};
// use venues::{binance::BinanceBook, coinbase::CoinbaseBook, upbit::UpbitBook, venue_traits::VenueFunctionality};
use venues::binance::BinanceBook;
use venues::coinbase::CoinbaseBook;
use venues::upbit::UpbitBook;
use venues::venue_traits::VenueFunctionality;
use rand::{Rng, random};


type MinNonNan = Reverse<NotNan<f64>>;
fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    // env::set_var("API_KEY", "");
    // env::set_var("API_SECRET", "");

    // let binance: BinanceBook = BinanceBook {
    //    name: String::from("binance"),
    //    base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
    //    symbol: String::from("BTCUSDT"),
    //    method: String::from("depth"),
    //    limit: 50,
    // };

    // binance.subscribe();

    // let coinbase: CoinbaseBook = CoinbaseBook {
    //    name: String::from("coinbase"),
    //    base_ws: String::from("wss://advanced-trade-ws.coinbase.com"),
    //    api_key: CoinbaseBook::get_api_key(),
    //    secret: CoinbaseBook::get_api_secret(),
    //    product_ids: vec![String::from("BTC-USD")],
    //    channel: String::from("level2"),
    // };

    // coinbase.subscribe();

    // let upbit: UpbitBook = UpbitBook {
    //    name: String::from("upbit"),
    //    base_ws: String::from("wss://api.upbit.com/websocket/v1"),
    //    channel: String::from("orderbook"),
    //    codes: vec![String::from("USDT-BTC")],
    // };

    // upbit.subscribe();
    
    

    // let mut bst: BTreeMap<MinNonNan, Limit> = BTreeMap::new();

    // for _ in 1..10000000{
    //     let limit = Limit {
    //         limit_price: to_float(rand::thread_rng().gen_range(0.0..=1.0)),
    //         total_volume: to_float(rand::thread_rng().gen_range(0.0..=1.0)),
    //         ..Default::default()
    //     };
    //     bst.insert(limit.limit_price, limit);
    // }

}
