// use ureq::*;
use std::error::Error;
use serde::Deserialize;


pub fn call_cardano_req() {
  let b = cardano_req();
  println!("{:?}", b);
}