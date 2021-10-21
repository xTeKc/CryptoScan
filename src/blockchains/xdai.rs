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
  let resp = ureq::get(req).call()?.into_string()?;

  let cryptos: Cryptos = serde_json::from_str(&resp)?;

  dbg!(cryptos);

  todo()
}

pub fn call_xdai_req() {

}