use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;

pub fn anyhow_test(a: StreamSink<Undurchsichtiger>) {
    todo!()
}

#[frb(opaque)]
pub struct Undurchsichtiger {
    a: std::fs::File,
}

// TODO
// impl Undurchsichtiger {
//     pub fn read(&self) -> Vec<u8> {
//         todo!()
//     }
// }
