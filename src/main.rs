mod orderbook;
mod utils;
mod venues;
use digest::typenum::Or;
use orderbook::{Book, Limit};
use std::env;
// use venues::{binance::BinanceBook, coinbase::CoinbaseBook, upbit::UpbitBook, venue_traits::VenueFunctionality};
use venues::binance::BinanceBook;
use venues::coinbase::CoinbaseBook;
use venues::upbit::UpbitBook;
use venues::venue_traits::VenueFunctionality;

fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    //env::set_var("API_KEY", "");
    //env::ser_var("API_SECRET", "");

    //let binance: BinanceBook = BinanceBook {
    //    name: String::from("binance"),
    //    base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
    //    symbol: String::from("BTCUSDT"),
    //    method: String::from("depth"),
    //    limit: 50,
    //};

    // binance.subscribe();

    //let coinbase: CoinbaseBook = CoinbaseBook {
    //    name: String::from("coinbase"),
    //    base_ws: String::from("wss://advanced-trade-ws.coinbase.com"),
    //    api_key: CoinbaseBook::get_api_key(),
    //    secret: CoinbaseBook::get_api_secret(),
    //    product_ids: vec![String::from("ETH-USD"), String::from("BTC-USD")],
    //    channel: String::from("level2"),
    //};

    // coinbase.subscribe();

    //let upbit: UpbitBook = UpbitBook {
    //    name: String::from("upbit"),
    //    base_ws: String::from("wss://api.upbit.com/websocket/v1"),
    //    channel: String::from("orderbook"),
    //    codes: vec![String::from("USDT-BTC")],
    //};

    // upbit.subscribe();
    //
    //

    let mut book = Book::new();

    let mut node = Box::new(Limit::new(1.002, 2.0));
    node.left_child = Some(Box::new(Limit::new(0.8, 1.0)));
    node.right_child = Some(Box::new(Limit::new(1.8, 0.2)));
    if let Some(child) = node.left_child.as_mut() {
        child.left_child = Some(Box::new(Limit::new(0.5, 0.3)));
    }
    if let Some(child) = node.right_child.as_mut() {
        child.right_child = Some(Box::new(Limit::new(2.5, 1.0)));
    }


    println!("Original tree");
    Limit::print_tree(&Some(node.clone()), "", false);

    node = Limit::insert(&mut Some(node), 3.0, 2.0, &mut book, true).unwrap();
    node = Limit::insert(&mut Some(node), 4.0, 2.0, &mut book, true).unwrap();
    node = Limit::insert(&mut Some(node), 5.0, 2.0, &mut book, true).unwrap();
    node = Limit::insert(&mut Some(node), 6.0, 2.0, &mut book, true).unwrap();

    println!("Removing 1.002");
    Limit::remove(&mut Some(node.clone()), 1.002);
}
