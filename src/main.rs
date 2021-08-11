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
    cardano::call_cardano_req();
    ethereum::call_ethereum_req();
    fantom::call_fantom_req();
    harmony::call_harmony_req();
    huobi::call_huobi_req();
    polygon::call_polygon_req();
    solana::call_solana_req();
    xdai::call_xdai_req();
}



pub fn binance_bc() {
    let bsc = //implement UI dropdown selection
}

pub fn cardano_bc() {
    let ada = //implement UI dropdown selection
}

pub fn ethereum_bc() {
    let eth = //implement UI dropdown selection
}

pub fn fantom_bc() {

}

pub fn harmony_bc() {

}

pub fn huobi_bc() {

}

pub fn polygon_bc() {

}

pub fn solana_bc() {

}

pub fn xdai_bc() {

}