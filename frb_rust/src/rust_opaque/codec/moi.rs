#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_moi_rust_opaque_codec_def {
    () => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct MoiRustOpaqueCodec;

        impl<T: ?Sized + 'static + MapBasedArcValue> BaseRustOpaqueCodec<T> for MoiRustOpaqueCodec {
            type Arc = MapBasedArc<T>;
        }
    };
}
