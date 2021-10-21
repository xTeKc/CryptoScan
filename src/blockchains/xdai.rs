// use ureq::*;
use std::error::Error;
use serde::Deserialize;

pub struct Cryptos {
  cryptos: Vec<Crypto>
}

pub struct Crypto {
  // name: String,
  symbol: String,
  marketcap: u32
}

pub fn xdai_req(req: &str) -> Result<Cryptos, Box<dyn Error>> {

}

pub fn call_xdai_req() {

}