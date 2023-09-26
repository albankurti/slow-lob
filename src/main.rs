mod orderbook;
mod utils;
mod venues;
use orderbook::Book;
use utils::{print,export_txt};
use std::sync::{Arc, Mutex};
use std::env;
use venues::binance::BinanceBook;
use venues::coinbase::CoinbaseBook;
use venues::upbit::UpbitBook;
use venues::venue_traits::VenueFunctionality;


fn main() {
    let binance: BinanceBook = BinanceBook {
       name: String::from("binance"),
       base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
       symbol: String::from("BTCUSDT"),
       method: String::from("depth"),
       limit: 50,
    };

    env::set_var("API_KEY", "");
    env::set_var("API_SECRET", "");
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


    let buy_tree = Arc::clone(&buy_tree_original);
    let sell_tree = Arc::clone(&sell_tree_original);
    let handle_binance = std::thread::spawn(move || {
        binance.subscribe(&mut buy_tree.lock().unwrap(), &mut sell_tree.lock().unwrap());
    });

    let buy_tree = Arc::clone(&buy_tree_original);
    let sell_tree = Arc::clone(&sell_tree_original);
    let handle_coinbase = std::thread::spawn(move || {
        coinbase.subscribe(&mut buy_tree.lock().unwrap(), &mut sell_tree.lock().unwrap());
    });

    handle_coinbase.join().unwrap();
    handle_binance.join().unwrap();

    // Access book's buy_tree and sell_tree after threads have completed.
    println!("");
    for (key, value) in &*buy_tree_original.lock().unwrap() {
        if value.volumes.len() > 1 {
            println!("Key: {:?}, Total Volume: {:?}", key, value.total_volume);
            for (volume, origin) in &value.volumes {
                println!("  Volume: {:?}, Origin: {:?}", volume, origin);
            }
        } 
    }
    println!("");
    for (key, value) in &*sell_tree_original.lock().unwrap() {
        if value.volumes.len() > 1 {
            println!("Key: {:?}, Total Volume: {:?}", key, value.total_volume);
            for (volume, origin) in &value.volumes {
                println!("  Volume: {:?}, Origin: {:?}", volume, origin);
            }
        } 
    }
}