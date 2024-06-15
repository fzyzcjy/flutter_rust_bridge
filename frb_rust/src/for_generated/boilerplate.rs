// Why put these code as boilerplate instead of putting in frb_rust crate directly?
// Because we need to avoid the Rust's `orphan rule`.
#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_boilerplate {
    (
        default_stream_sink_codec = $default_stream_sink_codec:ident,
        default_rust_opaque = $default_rust_opaque:ident,
        default_rust_auto_opaque = $default_rust_auto_opaque:ident,
    ) => {
        $crate::frb_generated_wrapper_types!();
        $crate::frb_generated_moi_arc_def!();
        $crate::frb_generated_rust_opaque_dart2rust!();
        $crate::frb_generated_rust_opaque_def!(default_rust_opaque = $default_rust_opaque);
        $crate::frb_generated_rust_auto_opaque_def!(
            default_rust_auto_opaque = $default_rust_auto_opaque
        );
        $crate::frb_generated_cst_codec!();
        $crate::frb_generated_sse_codec!();
        $crate::frb_generated_stream_sink!(default_stream_sink_codec = $default_stream_sink_codec);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_wrapper_types {
    () => {
        #[doc(hidden)]
        pub(crate) struct FrbWrapper<T>(T);

        // This is a blanket implementation to match previous behavior
        // where codegen is putting #[derive(Clone)] on all concrete wrapper types
        // this is surely used in the same way previous #[derive(Clone)] was used
        // frb-coverage:ignore-start
        impl<T: Clone> Clone for FrbWrapper<T> {
            fn clone(&self) -> Self {
                FrbWrapper(self.0.clone())
            }
        }
        // frb-coverage:ignore-end

        // PartialEq is required to implement Eq, and HashSet requires both Eq and Hash
        // It looks like HashSet is not calling Eq during the test suite
        // frb-coverage:ignore-start
        impl<T: PartialEq> PartialEq for FrbWrapper<T> {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }
        // frb-coverage:ignore-end

        impl<T: Eq> Eq for FrbWrapper<T> {}

        impl<T: std::hash::Hash> std::hash::Hash for FrbWrapper<T> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }
        }

        impl<T> From<T> for FrbWrapper<T> {
            fn from(t: T) -> Self {
                FrbWrapper(t)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_cst_codec {
    () => {
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_sse_codec {
    () => {
        pub trait SseDecode {
            fn sse_decode(deserializer: &mut $crate::for_generated::SseDeserializer) -> Self;
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
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_stream_sink {
    (default_stream_sink_codec = $default_stream_sink_codec:ident) => {
        #[derive(Clone)]
        pub struct StreamSink<
            T,
            Rust2DartCodec: $crate::for_generated::BaseCodec = $crate::for_generated::$default_stream_sink_codec,
        > {
            base: $crate::for_generated::StreamSinkBase<T, Rust2DartCodec>,
        }

        impl<T, Rust2DartCodec: $crate::for_generated::BaseCodec> StreamSink<T, Rust2DartCodec> {
            pub fn deserialize(raw: String) -> Self {
                Self { base: $crate::for_generated::StreamSinkBase::deserialize(raw) }
            }
        }

        impl<T> StreamSink<T, $crate::for_generated::DcoCodec> {
            pub fn add<T2>(&self, value: T) -> Result<(), $crate::Rust2DartSendError>
            where
                T: $crate::IntoIntoDart<T2>,
                T2: $crate::IntoDart,
            {
                self.add_raw($crate::for_generated::Rust2DartAction::Success, value)
            }

            pub fn add_error<TR, T2>(&self, value: TR) -> Result<(), $crate::Rust2DartSendError>
            where
                TR: $crate::IntoIntoDart<T2>,
                T2: $crate::IntoDart,
            {
                self.add_raw($crate::for_generated::Rust2DartAction::Error, value)
            }

            fn add_raw<TR, T2>(&self, action: $crate::for_generated::Rust2DartAction, value: TR) -> Result<(), $crate::Rust2DartSendError>
            where
                TR: $crate::IntoIntoDart<T2>,
                T2: $crate::IntoDart,
            {
                self.base.add_raw($crate::for_generated::DcoCodec::encode(
                    action,
                    value.into_into_dart(),
                ))
            }
        }

        impl<T> StreamSink<T, $crate::for_generated::SseCodec>
        where
            T: SseEncode,
        {
            pub fn add(&self, value: T) -> Result<(), $crate::Rust2DartSendError> {
                self.add_raw($crate::for_generated::Rust2DartAction::Success, value)
            }

            pub fn add_error<TR: SseEncode>(&self, value: TR) -> Result<(), $crate::Rust2DartSendError> {
                self.add_raw($crate::for_generated::Rust2DartAction::Error, value)
            }

            pub fn add_raw<TR: SseEncode>(&self, action: $crate::for_generated::Rust2DartAction, value: TR) -> Result<(), $crate::Rust2DartSendError> {
                self.base.add_raw($crate::for_generated::SseCodec::encode(
                    action,
                    |serializer| value.sse_encode(serializer),
                ))
            }
        }

        // Ignore coverage since unreachable
        // frb-coverage:ignore-start
        impl<T, Rust2DartCodec: $crate::for_generated::BaseCodec> $crate::IntoIntoDart<StreamSink<T, Rust2DartCodec>> for StreamSink<T, Rust2DartCodec> {
            fn into_into_dart(self) -> StreamSink<T, Rust2DartCodec> {
                unreachable!()
            }
        }

        impl<T, Rust2DartCodec: $crate::for_generated::BaseCodec> $crate::IntoDart for StreamSink<T, Rust2DartCodec> {
            fn into_dart(self) -> $crate::for_generated::DartAbi {
                unreachable!()
            }
        }
        // frb-coverage:ignore-end
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_default_handler {
    () => {
        #[cfg(not(target_family = "wasm"))]
        $crate::for_generated::lazy_static! {
            pub static ref FLUTTER_RUST_BRIDGE_HANDLER:$crate::DefaultHandler<$crate::for_generated::SimpleThreadPool> = {
                assert_eq!(
                    FLUTTER_RUST_BRIDGE_CODEGEN_VERSION,
                    flutter_rust_bridge::for_generated::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
                    "Please ensure flutter_rust_bridge's codegen ({}) and runtime ({}) versions are the same",
                    FLUTTER_RUST_BRIDGE_CODEGEN_VERSION,
                    flutter_rust_bridge::for_generated::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
                );

                $crate::DefaultHandler::new_simple(Default::default())
            };
        }

        #[cfg(target_family = "wasm")]
        thread_local! {
            pub static THREAD_POOL: $crate::for_generated::SimpleThreadPool = Default::default();
        }

        #[cfg(target_family = "wasm")]
        $crate::for_generated::lazy_static! {
            pub static ref FLUTTER_RUST_BRIDGE_HANDLER: $crate::DefaultHandler<&'static std::thread::LocalKey<$crate::for_generated::SimpleThreadPool>>
                = $crate::DefaultHandler::new_simple(&THREAD_POOL);
        }
    }
}
