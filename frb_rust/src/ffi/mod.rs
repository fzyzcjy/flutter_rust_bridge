#[cfg(wasm)]
pub type DartAbi = wasm_bindgen::JsValue;
#[cfg(not(wasm))]
pub type DartAbi = allo_isolate::ffi::DartCObject;

#[cfg(wasm)]
pub trait IntoDart {
    fn into_dart(self) -> DartAbi;
}
#[cfg(not(wasm))]
pub use allo_isolate::IntoDart;

#[cfg(wasm)]
pub type MessagePort = web::PortLike;
#[cfg(not(wasm))]
pub type MessagePort = i64;

#[cfg(wasm)]
pub mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
pub type Channel = allo_isolate::Isolate;

#[cfg(not(wasm))]
pub mod io;
#[cfg(not(wasm))]
pub use io::*;
