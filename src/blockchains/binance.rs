use ureq;

// pub fn binance_req() -> Result<(), ureq::Error> {
//   let req: String = ureq::get("https://api.coingecko.com/api/v3/ping")
//   .set("Example-Header", "header value")
//   .call()?
//   .into_string()?;
//   println!("{:?}", req);
// Ok(())
// }

// pub fn binance_req() {
//   let req = ureq::get("https://api.coingecko.com/api/v3/ping");
//   println!("{:?}", req);
// }

pub fn call_binance_req() {
  let b = binance_req();
  println!("{:?}", b);
}