// use ureq::*;
use std::error::Error;
use serde::Deserialize;

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

pub fn xdai_req(req: &str) -> Result<Vec<Crypto>, Box<dyn Error>> {
  let resp = ureq::get(req).call()?.into_string()?;

  let cryptos: Cryptos = serde_json::from_str(&resp)?;

  dbg!(cryptos);

  todo!()
}

pub fn call_xdai_req() {
  let req = "https://api.coingecko.com/api/v3/global";
  let cryptos = xdai_req(req);
  dbg!(cryptos);
}