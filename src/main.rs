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

use iced::{
    button,
    futures,
    image.
    Align,
    Application,
    Button,
    Column,
    Command,
    Container,
    Element,
    Image,
    Length,
    Row,
    Settings,
    Text,    
};



pub fn ui() -> iced::Result {
    Crypdatrs::run(Settings::default())
}

#[derive(Debug)]
enum Crypdatrs {
    Loading,
    Loaded {
        //crypdatrs: CrypDatRs,
        //search: button::State,
    },
    Errored {
        error: Error,
        try_again: button::State,
    },
}

#[derive(Debug, Clone)]
enum Message {
    //CrypDatRsFound(Result<CrypDatRs, Error>),
    Search,
}

impl Application for CrypDatRs {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (CrypDatRs, Command<Message>) {
        (
            CrypDatRs::Loading,
            Command::perform(CrypDatRs::search(), Message::CrypDatRsFound),
        )
    }
}


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
    let bsc_dropdown = //implement UI dropdown selection
}

pub fn cardano_bc() {
    let ada_dropdown = //implement UI dropdown selection
}

pub fn ethereum_bc() {
    let eth_dropdown = //implement UI dropdown selection
}

pub fn fantom_bc() {
    let ftm_dropdown = //imimplement UI dropdown selection
}

pub fn harmony_bc() {
    let hmy_dropdown = //implement UI dropdown selection
}

pub fn huobi_bc() {
    let huo_dropdown = //implement UI dropdown selection
}

pub fn polygon_bc() {
    let poly_dropdown = //implement UI dropdown selection
}

pub fn solana_bc() {
    let sol_dropdown = //implement UI dropdown selection
}

pub fn xdai_bc() {
    let xdai_dropdown = //implement UI dropdown selection
}