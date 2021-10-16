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

pub fn fantom_req(req: &str) -> Result<Cryptos, Box<dyn Error>> {
  let resp = ureq::get(req).call()?.into_string()?;

  let cryptos: Cryptos = serde_json::from_str(&resp)?;

  dbg!(cryptos);

  todo!()
}


pub fn call_fantom_req() {
  let req = "https://api.coingecko.com/api/v3/global";
  let cryptos = fantom_req(req);
  dbg!(cryptos);
}