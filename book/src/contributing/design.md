# Overall design

> This doc is still WIP. Tracking issue: https://github.com/fzyzcjy/flutter_rust_bridge/issues/593

## Folder structure

* `frb_codegen`: Code generator. It inputs `api.rs` and outputs Rust and Dart code files.
* `frb_example`: Examples.
  * `pure_dart`: Not only an example, but, more importantly, serves as end-to-end tests.
  * `with_flutter`: Example with integration into Flutter.
  * `pure_dart_multi`: Demonstrate multi-file usage.
* `frb_dart`: Support library for Dart - to be imported by users.
* `frb_rust`: Support library for Rust - to be imported by users.
* `frb_macros`: Indeed part of `frb_rust`. <small>It is a separate package simply because limitation of proc macros.</small>
* `book`: The documentation.
* `.github`: GitHub-related.
  * `workflows/ci.yaml`: Definition of CI workflows.

## Code-generator structure

The pipeline is as follows:

```
----------    src/parser    ----------    src/generator     ---------------
| api.rs | ---------------> | src/ir | -------------------> | Rust & Dart |
----------                  ----------                      ---------------
```

* The input, `api.rs` in the figure, is the user-provided handwritten Rust code.
* The parser (`src/parser`) converts the input code (indeed [syn](https://crates.io/crates/syn) tree) into IR.
* IR (`src/ir`), or internal representation, is a data structure that represents the information of the code that we are interested in.
* The generator (`src/generator`) converts the IR into final outputs. More specifcially, as you can probably guess, `src/generator/dart` generates Dart code, `src/generator/rust` is for Rust code, and `src/generator/c` is for (a bit of) C code.
* The outputs (`Rust & Dart` in the figure) are written to corresponding files.

## Data flow

Let us see what happens when a function is called.

Suppose a user calls a (generated) Dart function `func({required String str})`. Then, the following happens:

1. The generated Dart function, `func({required String str})`, convert "*Dart api data*" (i.e. the data that user really provides) into "*Dart wire data*" (i.e. the data that will really pass between Dart and Rust). More specifically, it calls `_api2wire_String(str)` and get a `ffi.Pointer<wire_uint_8_list>` (because `String`s use `pub struct wire_uint_8_list { ptr: *mut u8, len: i32 }` under the hood).
2. Now we call the Dart version of `wire_func`, with low-level data like `wire_uint_8_list`. We have used our codegen to create a Rust `wire_func` function, and use `cbindgen` to generate the corresponding C function, and use `ffigen` to get the cooresponding Dart function. Here, we call the Dart version of `wire_func`. Since Dart FFI and Rust FFI is C-compatible, it seamlessly calls the Rust version of `wire_func`. Notice that, since we are utilizing C-compatible functions (and it is the only feasible way), we can only pass around low-level things like pointers, instead of high-level and safe things.
2. Surely, the Rust `wire_func` is called. The function uses `.wire2api()` to convert "*Rust wire data*" (`wire_uint_8_list` here) into "*Rust api data*" (`String` here, i.e. data that users really use). 
2. The `FLUTTER_RUST_BRIDGE_HANDLER` is called with "*Rust api data*". That handler is user-customizable, so users may provide their own implementation other than the default thread-pool, etc. By default, we use a thread pool, and we call the user-written `func` Rust function in `api.rs`.
2. The user-written `fn func(str: String) -> String { ... }` is called, and we get a return value.
2. The return value, a `String`, is posted to the Dart side. It is done by the Dart-provided API, [`Dart_PostCObject`](https://github.com/dart-lang/sdk/blob/fd0d3b254690007d0ebc84175f30fa7d7491ec3e/runtime/include/dart_native_api.h#L124), which let us provide C structs and it will automatically become Dart data on the other side. We use the Rust-safe wrapper `allo-isolate` for it. We deliberately choose this, because this enables Dart code to be *async* instead of sync.
2. On the Dart side, we now see some Dart objects (indeed "*Dart wire data*"). We use functions like `_wire2api_SomeType` to convert it to the final "*Dart api data*". Notice this "wire2api" is on *Dart* side, so it means "*Dart* wire data to *Dart* api data", and is different from the one above which is for Rust. For example, since `Dart_PostCObject` does not provide a way to construct arbitrary structs(classes), we have to pass Rust structs as lists, and use the `wire2api` to convert them to corresponding Dart classes.
2. The final result value is provided as return value of the Dart function, `func`, that the user called just now. A function call finishes!

## Memory safety

How is memory safety implemented? This is a case-by-case problem. For example, suppose we want to see how a `String` is safely passed from Dart to Rust. Then, we need to examine the Dart `_api2wire_String` and the Rust `.wire2api()` for it.

Indeed `String` is implemented by delegating to `Vec<u8>`, so we need to see code related to String as well as `Vec<u8>`. By simply clicking a few times and jump around code, we will see that:

```dart
ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
  return _api2wire_uint_8_list(utf8.encoder.convert(raw));
}

ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
  final ans = inner.new_uint_8_list_0(raw.length);
  ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
  return ans;
}
```

and

```rust,noplayground
impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}
```

In other words, String (or `Vec<u8>`) is converted to a raw struct with pointer and length field. The memory is manipulated carefully so there is no leak or double free.

We use Valgrind to check as well, and I use it in production environment without problems, so no worries about memory problems :)

## Want to know more? Tell me

What do you want to know? Feel free to create an issue in GitHub, and I will tell more :)
