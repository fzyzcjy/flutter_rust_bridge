use crate::frb_generated::{CstDecode, FLUTTER_RUST_BRIDGE_HANDLER};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::rust_async::RwLock;
use flutter_rust_bridge::{Handler, NomRustOpaqueCodec, RustOpaque};

pub mod api;
mod auxiliary;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

fn wire_rust_auto_opaque_callable_arg_twin_rust_async_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    arg: impl CstDecode<
        flutter_rust_bridge::RustOpaque<
            flutter_rust_bridge::for_generated::rust_async::RwLock<
                Box<dyn Fn(String) -> String + Send + Sync>,
            >,
            NomRustOpaqueCodec,
        >,
    >,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_async::<flutter_rust_bridge::for_generated::DcoCodec, _, _, _>(
        flutter_rust_bridge::for_generated::TaskInfo {
            debug_name: "rust_auto_opaque_callable_arg_twin_rust_async",
            port: Some(port_),
            mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
        },
        move || {
            let api_arg: RustOpaque<_, NomRustOpaqueCodec> = arg.cst_decode();
            move |context| async move {
                let api_arg = api_arg.rust_auto_opaque_decode_async_owned().await;
                panic!()
            }
        },
    )
}
