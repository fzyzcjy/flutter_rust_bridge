// Why put these code as boilerplate instead of putting in frb_rust crate directly?
// Because we need to avoid the Rust's `orphan rule`.
#[macro_export]
macro_rules! frb_generated_boilerplate {
    () => {
        // -------------------------- CstCodec ------------------------

        pub trait CstDecode<T> {
            fn cst_decode(self) -> T;
        }

        impl<T, S> CstDecode<Option<T>> for *mut S
        where
            *mut S: CstDecode<T>,
        {
            fn cst_decode(self) -> Option<T> {
                (!self.is_null()).then(|| self.cst_decode())
            }
        }

        // -------------------------- SseCodec ------------------------

        pub trait SseDecode {
            fn sse_decode(
                deserializer: &mut flutter_rust_bridge::for_generated::SseDeserializer,
            ) -> Self;
        }

        pub trait SseEncode {
            fn sse_encode(self, serializer: &mut flutter_rust_bridge::for_generated::SseSerializer);
        }

        fn transform_result_sse<T, E>(
            raw: Result<T, E>,
        ) -> Result<
            flutter_rust_bridge::for_generated::Rust2DartMessageSse,
            flutter_rust_bridge::for_generated::Rust2DartMessageSse,
        >
        where
            T: SseEncode,
            E: SseEncode,
        {
            use flutter_rust_bridge::for_generated::{Rust2DartAction, SseCodec};

            match raw {
                Ok(raw) => Ok(SseCodec::encode(Rust2DartAction::Success, |serializer| {
                    raw.sse_encode(serializer)
                })),
                Err(raw) => Err(SseCodec::encode(Rust2DartAction::Error, |serializer| {
                    raw.sse_encode(serializer)
                })),
            }
        }

        // -------------------------- StreamSink ------------------------

        pub struct StreamSink<T, Rust2DartCodec: flutter_rust_bridge::for_generated::BaseCodec = flutter_rust_bridge::for_generated::DcoCodec> {
            base: flutter_rust_bridge::for_generated::StreamSinkBase<T, Rust2DartCodec>,
        }

        impl<T, Rust2DartCodec: flutter_rust_bridge::for_generated::BaseCodec> StreamSink<T, Rust2DartCodec> {
            pub fn new(base: flutter_rust_bridge::for_generated::StreamSinkBase<T, Rust2DartCodec>) -> Self {
                Self { base }
            }

            pub fn add(&self, value: T) -> bool {
                self.base.add(value)
            }

            pub fn close(&self) -> bool {
                self.base.close()
            }
        }
    };
}
