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
            fn sse_decode(deserializer: &mut $crate::for_generated::SseDeserializer) -> Self;

            // just syntax sugar
            fn sse_decode_single(message: $crate::for_generated::Dart2RustMessageSse) -> Self {
                let mut deserializer = $crate::for_generated::SseDeserializer::new(message);
                let ans = Self::sse_decode(&mut deserializer);
                deserializer.end();
                ans
            }
        }

        pub trait SseEncode {
            fn sse_encode(self, serializer: &mut $crate::for_generated::SseSerializer);
        }

        fn transform_result_sse<T, E>(
            raw: Result<T, E>,
        ) -> Result<
            $crate::for_generated::Rust2DartMessageSse,
            $crate::for_generated::Rust2DartMessageSse,
        >
        where
            T: SseEncode,
            E: SseEncode,
        {
            use $crate::for_generated::{Rust2DartAction, SseCodec};

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

        #[derive(Clone)]
        pub struct StreamSink<
            T,
            Rust2DartCodec: $crate::for_generated::BaseCodec = $crate::for_generated::DcoCodec,
        > {
            base: $crate::for_generated::StreamSinkBase<T, Rust2DartCodec>,
        }

        impl<T, Rust2DartCodec: $crate::for_generated::BaseCodec> StreamSink<T, Rust2DartCodec> {
            pub fn new(base: $crate::for_generated::StreamSinkBase<T, Rust2DartCodec>) -> Self {
                Self { base }
            }

            pub fn close(&self) -> bool {
                self.base.close()
            }
        }

        impl<T> StreamSink<T, $crate::for_generated::DcoCodec> {
            pub fn add<T2>(&self, value: T) -> bool
            where
                T: $crate::IntoIntoDart<T2>,
                T2: $crate::IntoDart,
            {
                self.base.add($crate::for_generated::DcoCodec::encode(
                    $crate::for_generated::Rust2DartAction::Success,
                    value.into_into_dart(),
                ))
            }
        }

        impl<T> StreamSink<T, $crate::for_generated::SseCodec>
        where
            T: SseEncode,
        {
            pub fn add(&self, value: T) -> bool {
                self.base.add($crate::for_generated::SseCodec::encode(
                    $crate::for_generated::Rust2DartAction::Success,
                    |serializer| value.sse_encode(serializer),
                ))
            }
        }
    };
}
