//! Manages receiving and sending values across the FFI boundary.

use std::marker::PhantomData;

/// The representation of a Dart object outside of the Dart heap.
///
/// Its implementation lies with the Dart language and therefore should not be
/// depended on to be stable.
pub use crate::ffi::*;

/// A wrapper around a Dart [`Isolate`].
#[derive(Clone)]
pub struct Rust2Dart {
    pub(crate) isolate: Isolate,
}

const RUST2DART_ACTION_SUCCESS: i32 = 0;
const RUST2DART_ACTION_ERROR: i32 = 1;
const RUST2DART_ACTION_CLOSE_STREAM: i32 = 2;

// api signatures is similar to Flutter Android's callback https://api.flutter.dev/javadoc/io/flutter/plugin/common/MethodChannel.Result.html
impl Rust2Dart {
    /// Create a new wrapper from a raw port.
    pub fn new(port: MessagePort) -> Self {
        Rust2Dart {
            isolate: Isolate::new(port),
        }
    }

    /// Send a success message back to the specified port.
    pub fn success(&self, result: impl IntoDart) -> bool {
        self.isolate.post(vec![
            RUST2DART_ACTION_SUCCESS.into_dart(),
            result.into_dart(),
        ])
    }

    /// Send an error back to the specified port.
    pub fn error(&self, error_code: String, error_message: String) -> bool {
        self.error_full(error_code, error_message, ())
    }

    /// Send a detailed error back to the specified port.
    pub fn error_full(
        &self,
        error_code: String,
        error_message: String,
        error_details: impl IntoDart,
    ) -> bool {
        self.isolate.post(vec![
            RUST2DART_ACTION_ERROR.into_dart(),
            error_code.into_dart(),
            error_message.into_dart(),
            error_details.into_dart(),
        ])
    }

    /// Close the stream and ignore further messages.
    pub fn close_stream(&self) -> bool {
        self.isolate
            .post(vec![RUST2DART_ACTION_CLOSE_STREAM.into_dart()])
    }
}

/// A callback that receives the return value of Rust functions.
pub struct TaskCallback {
    rust2dart: Rust2Dart,
}

impl TaskCallback {
    /// Create a new callback from a port wrapper.
    pub fn new(rust2dart: Rust2Dart) -> Self {
        Self { rust2dart }
    }

    /// Create a new [StreamSink] of the specified type.
    pub fn stream_sink<T: IntoDart>(&self) -> StreamSink<T> {
        StreamSink::new(self.rust2dart.clone())
    }
}

#[derive(Clone)]
pub struct Channel(pub String);

impl Channel {
    #[cfg(wasm)]
    pub fn port(&self) -> MessagePort {
        PortLike::broadcast(&self.0)
    }
}

// #[cfg(wasm)]
// #[derive(Default)]
// struct Signal {
//     started: std::sync::Mutex<bool>,
//     cvar: std::sync::Condvar,
// }

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSink<T: IntoDart> {
    #[cfg(not(wasm))]
    rust2dart: Rust2Dart,
    /// On WASM, [`Rust2Dart`] is not [`Send`] so we only
    /// carry the name of the broadcast channel.
    #[cfg(wasm)]
    channel: Channel,
    // #[cfg(wasm)]
    // signal: std::sync::Arc<Signal>,
    _phantom_data: PhantomData<T>,
}

// #[cfg(wasm)]
// lazy_static::lazy_static! {
//     static ref SINK_INIT: std::sync::Mutex<std::collections::HashMap<String, std::sync::Arc<Signal>>> = Default::default();
// }

// #[cfg(wasm)]
// #[doc(hidden)]
// #[wasm_bindgen]
// pub fn __start_streamsink(name: String) {
//     use crate::pool::WorkerPool;
//     thread_local! {
//         static CONTROL_POOL: WorkerPool = WorkerPool::new(1, script_path().unwrap()).unwrap();
//     }

//     CONTROL_POOL.with(|pool| {
//         pool.run(transfer!(|| {
//             if let Some(signal) = SINK_INIT.lock().unwrap().get(&name) {
//                 *signal.started.lock().unwrap() = true;
//                 signal.cvar.notify_all();
//             } else {
//                 console_error(&format!("StreamSink was not registered in time: {}", name));
//             }
//         }))
//         .unwrap();
//     });
// }

impl<T: IntoDart> StreamSink<T> {
    /// Create a new sink from a port wrapper.
    pub fn new(rust2dart: Rust2Dart) -> Self {
        #[cfg(wasm)]
        let name = rust2dart
            .isolate
            .broadcast_name()
            .expect("Not a BroadcastChannel");
        Self {
            #[cfg(not(wasm))]
            rust2dart,
            #[cfg(wasm)]
            channel: Channel(name),
            // #[cfg(wasm)]
            // signal: {
            //     let mut signals = SINK_INIT.lock().unwrap();
            //     if signals.contains_key(&name) {
            //         drop(signals);
            //         panic!("Name collision: {}", name);
            //     }
            //     signals.entry(name).or_default().clone()
            // },
            _phantom_data: PhantomData,
        }
    }

    fn rust2dart(&self) -> Rust2Dart {
        #[cfg(not(wasm))]
        return self.rust2dart.clone();

        #[cfg(wasm)]
        Rust2Dart::new(self.channel.port())
    }

    fn wait_init(&self) {
        // #[cfg(wasm)]
        // {
        //     let mut started = self.signal.started.lock().unwrap();
        //     while !*started {
        //         console_log("StreamSink waiting...");
        //         started = self.signal.cvar.wait(started).unwrap();
        //     }
        // }
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    ///
    /// On WASM platforms, will block the thread until the Dart thread gives
    /// the start signal.
    pub fn add(&self, value: T) -> bool {
        self.wait_init();
        self.rust2dart().success(value)
    }

    /// Close the stream and ignore further messages. Returns false when
    /// the stream could not be closed, or when it has already been closed.
    ///
    /// On WASM platforms, will block the thread until the Dart thread gives
    /// the start signal.
    pub fn close(&self) -> bool {
        self.wait_init();
        self.rust2dart().close_stream()
    }
}
