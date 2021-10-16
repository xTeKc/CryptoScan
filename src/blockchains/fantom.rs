// use ureq::*;
use std::error::Error;
use serde::Deserialize;

pub struct Cryptos {
  cryptos: Vec<Crypto>
}

pub fn fantom_req() -> Result<(), ureq::Error> {
  let req: String = ureq::get("https://api.coingecko.com/api/v3/ping")
  .set("Example-Header", "header value")
  .call()?
  .into_string()?;
  println!("{:?}", req);
Ok(())
}

pub fn call_fantom_req() {
  let b = fantom_req();
  println!("{:?}", b);
}