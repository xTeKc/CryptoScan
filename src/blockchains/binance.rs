use ureq::*;
use std::error::Error;

pub struct Cryptos {
  cryptos: Vec<Crypto>
}

pub struct Crypto {
  // name: String,
  symbol: String,
  marketcap: u32
}

pub fn binance_req(req: &str) -> Result<Cryptos, Box<dyn Error>> {
  let resp = ureq::get(req).call()?.into_string()?;

  dbg!(resp);

  todo!()
}

// pub fn binance_req() -> Result<(), ureq::Error> {
//   let req: String = ureq::get("https://api.coingecko.com/api/v3/global")
//   .set("Example-Header", "header value")
//   .call()?
//   .into_string()?;
//   println!("{:?}", req);
// Ok(())
// }

pub fn call_binance_req() {
  let req = "https://api.coingecko.com/api/v3/global";
  let cryptos = binance_req(req);
}