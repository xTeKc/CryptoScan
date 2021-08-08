use ureq;

pub fn solana_req() -> Result<(), ureq::Error> {
  let req: String = ureq::get("https://api.coingecko.com/api/v3/ping")
  .set("Example-Header", "header value")
  .call()?
  .into_string()?;
  println!("{:?}", req);
Ok(())
}

pub fn call_solana_req() {
  let b = solana_req();
  println!("{:?}", b);
}