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

}

pub fn call_ethereum_req() {
  let b = ethereum_req();
  println!("{:?}", b);
}