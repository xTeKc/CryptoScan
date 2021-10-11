use ureq::*;


pub struct Cryptos {
  cryptos: Vec<Crypto>
}

pub struct Crypto {
  // add data
}

pub fn binance_req() -> Result<(), ureq::Error> {
  let req: String = ureq::get("https://api.coingecko.com/api/v3/global")
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