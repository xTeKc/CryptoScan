pub mod binance;
pub mod binance_req;

pub fn binance_req() {
    match reqwest::get("https://api.coingecko.com/api/v3/ping")
        Ok(mut response) => {
            //Check if 200 ok
            if response.status() == reqwest::StatusCode::OK {
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text")
                }
            } else {
                println!("Response was not 200 Ok")
            }
        }
        Err(_) => println!("Could not make request!")
}