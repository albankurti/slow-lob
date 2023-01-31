mod venues;
mod exchange_traits;

use std::env;
use venues::{BinanceBook, CoinbaseBook, UpbitBook};
use exchange_traits::ExchangeFunctionality;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let binance: BinanceBook = BinanceBook {
        name: String::from("binance"),
        base_ws: String::from("wss://ws-api.binance.com:443/ws-api/v3"),
        symbol: String::from("BTCUSDT"),
        method: String::from("depth"),
        limit: 5,
    };

    // binance.subscribe();

    let coinbase: CoinbaseBook = CoinbaseBook {
        name: String::from("coinbase"),
        base_ws: String::from("wss://advanced-trade-ws.coinbase.com"),
        api_key: CoinbaseBook::get_api_key(),
        secret: CoinbaseBook::get_api_secret(),
        product_ids: vec![String::from("ETH-USD"), String::from("BTC-USD")],
        channel: String::from("level2"),
    };

    // coinbase.subscribe();

    let upbit: UpbitBook = UpbitBook {
        name: String::from("upbit"),
        base_ws: String::from("wss://api.upbit.com/websocket/v1"),
        channel: String::from("orderbook"),
        codes: vec![String::from("USDT-BTC")],
    };

    // upbit.subscribe();

}