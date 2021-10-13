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

pub fn ethereum_req() {
  let resp = ureq::get(req).call()?.into_string()?;

  let cryptos: Cryptos = serde_json::from_str(&resp)?;

}

pub fn call_ethereum_req() {
  let b = ethereum_req();
  println!("{:?}", b);
}