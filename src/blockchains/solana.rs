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

pub fn solana_req(req: &str) -> Result<Cryptos, Box<dyn Error>> {
  let resp = ureq::get(req).call()?.into_string()?;

}

pub fn call_solana_req() {

}