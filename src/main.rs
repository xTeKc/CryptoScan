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

}

pub fn cardano_bc() {

}

pub fn ethereum_bc() {

}

pub fn fantom_bc() {
    
}