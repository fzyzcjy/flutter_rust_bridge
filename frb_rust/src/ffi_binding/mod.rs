#[cfg(not(wasm))]
mod io;
#[cfg(not(wasm))]
pub use io::*;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[no_mangle]
pub extern "C" fn initialize_frb_rust() {
    // Nothing yet
}
