mod blockchains;
use blockchains::{
    binance,
    cardano,
    ethereum,
    fantom,
    harmony,
    huobi,
    polygon,
    solana,
    xdai
};


pub fn main() {
    binance::call_binance_req();
    // cardano::cardano_req();
    // ethereum::ethereum_req();
    // fantom::fantom_req();
    // harmony::harmony_req();
    // huobi::huobi_req();
    // polygon::polygon_req();
    // solana::solana_req();
    // xdai::xdai_req();
}
