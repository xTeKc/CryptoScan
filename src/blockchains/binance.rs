#![allow(unused_must_use)]

use std::error::Error;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Cryptos {
  cryptos: Vec<Crypto>
}

#[derive(Deserialize, Debug)]
pub struct Crypto {
  name: String,
  symbol: String,
  marketcap: u32
}

#[tokio::main]
pub async fn binance_req(req: &str) -> Result<Cryptos, Box<dyn Error>> {
  let resp = reqwest::get(req)
  .await?
  .text()
  .await?;
  
  println!("resp = {:?}", resp);
  
  let cryptos: Cryptos = serde_json::from_str(&resp)?;

  dbg!(cryptos);

  todo!()
}

pub fn call_binance_req() {
  let req = "https://api.coingecko.com/api/v3/global";
  let cryptos = binance_req(req);
  // dbg!(cryptos);
  println!("{req}");
}
