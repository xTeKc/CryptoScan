// use ureq::*;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Cryptos {
  cryptos: Vec<Crypto>
}

#[derive(Deserialize, Debug)]
pub struct Crypto {
  // name: String,
  symbol: String,
  marketcap: u32
}

pub fn polygon_req(req: &str) -> Result<Cryptos, Box<dyn Error>> {
  let resp = ureq::get(req).call()?.into_string()?;

}

pub fn call_polygon_req() {

}