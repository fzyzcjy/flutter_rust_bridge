#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
#[cfg(not(wasm))]
pub use io::*;

#[macro_export]
macro_rules! define_static_thread_pool {
    ($name:ident) => {
        #[cfg(not(target_family = "wasm"))]
        $crate::for_generated::lazy_static! {
            pub static ref $name: Mutex<$crate::for_generated::ThreadPool> = Mutex::new(Default::default());
        }

        #[cfg(target_family = "wasm")]
        thread_local! {
            pub static $name: $crate::for_generated::ThreadPool = Default::default();
        }
    };
}
