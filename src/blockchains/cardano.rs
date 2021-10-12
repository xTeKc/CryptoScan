// use ureq::*;
use std::error::Error;
use serde::Deserialize;

pub struct Cryptos {
  cryptos: Vec<Crypto>
}

pub fn call_cardano_req() {
  let b = cardano_req();
  println!("{:?}", b);
}