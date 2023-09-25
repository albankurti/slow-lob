mod orderbook;
mod utils;
mod venues;
use orderbook::Book;
use std::env;
use venues::binance::BinanceBook;
use venues::coinbase::CoinbaseBook;
use venues::upbit::UpbitBook;
use venues::venue_traits::VenueFunctionality;


fn main() {
    let mut book = Book::new();

    let binance: BinanceBook = BinanceBook {
       name: String::from("binance"),
       base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
       symbol: String::from("BTCUSDT"),
       method: String::from("depth"),
       limit: 50,
    };

    binance.subscribe(&mut book.buy_tree, &mut book.sell_tree);

    // env::set_var("API_KEY", "");
    // env::set_var("API_SECRET", "");

    // let coinbase: CoinbaseBook = CoinbaseBook {
    //    name: String::from("coinbase"),
    //    base_ws: String::from("wss://advanced-trade-ws.coinbase.com"),
    //    api_key: CoinbaseBook::get_api_key(),
    //    secret: CoinbaseBook::get_api_secret(),
    //    product_ids: vec![String::from("BTC-USD")],
    //    channel: String::from("level2"),
    // };

    // coinbase.subscribe(&mut book.buy_tree, &mut book.sell_tree);

    // let upbit: UpbitBook = UpbitBook {
    //    name: String::from("upbit"),
    //    base_ws: String::from("wss://api.upbit.com/websocket/v1"),
    //    channel: String::from("orderbook"),
    //    codes: vec![String::from("USDT-BTC")],
    // };

    // upbit.subscribe(&mut book.buy_tree, &mut book.sell_tree);

}
