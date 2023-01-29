use std::env;
mod websockets;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
}