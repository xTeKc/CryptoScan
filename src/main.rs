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
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};


fn gui() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);
    let mut but = Button::new(160, 210, 80, 40, "Search");
    let binance = binance::call_binance_req();

    wind.end();
    wind.show();
    
    let call = but.set_callback(move |_| frame.set_label("Hello world"));

    println!("{:?}, {:?}", call, binance);

    app.run().unwrap();
}


pub fn main() {
    // binance::call_binance_req();
    // cardano::call_cardano_req();
    // ethereum::call_ethereum_req();
    // fantom::call_fantom_req();
    // harmony::call_harmony_req();
    // huobi::call_huobi_req();
    // polygon::call_polygon_req();
    // solana::call_solana_req();
    // xdai::call_xdai_req();
    gui();
}



// pub fn binance_bc() {
//     let bsc_dropdown
// }

// pub fn cardano_bc() {
//     let ada_dropdown
// }

// pub fn ethereum_bc() {
//     let eth_dropdown
// }

// pub fn fantom_bc() {
//     let ftm_dropdown
// }

// pub fn harmony_bc() {
//     let hmy_dropdown
// }

// pub fn huobi_bc() {
//     let huo_dropdown
// }

// pub fn polygon_bc() {
//     let poly_dropdown
// }

// pub fn solana_bc() {
//     let sol_dropdown
// }

// pub fn xdai_bc() {
//     let xdai_dropdown
// }