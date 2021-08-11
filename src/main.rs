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
    let ftm = //implement UI dropdown selection
}

pub fn harmony_bc() {
    let hmy = //implement UI dropdown selection
}

pub fn huobi_bc() {
    let huo = //implement UI dropdown selection
}

pub fn polygon_bc() {
    let poly = //implement UI dropdown selection
}

pub fn solana_bc() {
    let sol = //implement UI dropdown selection
}

pub fn xdai_bc() {
    let xdai = //implement UI dropdown selection
}