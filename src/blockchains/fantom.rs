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

pub fn fantom_req() {
  
}


pub fn call_fantom_req() {
  let b = fantom_req();
  println!("{:?}", b);
}