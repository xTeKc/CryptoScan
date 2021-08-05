mod blockchains;
use std::io::{Read, Write};
use blockchains::binance;

fn main() {
    binance::binance_req();
}
