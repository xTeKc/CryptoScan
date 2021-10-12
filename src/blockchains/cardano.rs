use ureq;



pub fn call_cardano_req() {
  let b = cardano_req();
  println!("{:?}", b);
}