use ureq::*;
use std::env;

pub fn cli() {
  let args: Vec<String> = env::args().collect();
  let command = args[1].clone();

  println!("Command: {}", command);
}

pub fn binance_req() -> Result<(), ureq::Error> {
  let req: String = ureq::get("https://api.coingecko.com/api/v3/ping")
  .set("Example-Header", "header value")
  .call()?
  .into_string()?;
  println!("{:?}", req);
Ok(())
}

pub fn call_binance_req() {
  let b = binance_req();
  println!("{:?}", b);
}