# Codec

## Main codec

The "codec" here means how Dart data becomes accessible from the Rust side, and vise versa.
For example, suppose we have a Dart object of class `class MyClass { String name; }`.
Then, the chosen codec will do the job of constructing a Rust object of type `struct MyClass { name: String }`.
Similar things hold for Rust-to-Dart, and also hold for non-encodable types (opaque types), etc.

Currently, we have three codecs:

* **CST** (C-STruct):
Mimic how humans transfer the fields.
For example (simplified for demonstration),
create a C struct `struct MyClass { char* name_ptr; int name_arr_len; }` as the intermediate step.
* **DCO** (Dart_CObject-based):
Use the [Dart_CObject](https://github.com/dart-lang/sdk/blob/72f6db9261a7d0c96c5fc11ed4bd9f17ccd7d071/runtime/include/dart_native_api.h#L63)
as the intermediate step.
* **SSE** (Simple SErialization): Serialize everything into a byte buffer, and deserialize on the other side.

In addition, CST is implemented for Dart-to-Rust, DCO for Rust-to-Dart,
and SSE for both directions.

Currently, CST+DCO is the default choice. To use SSE instead, specify `#[frb(serialize)]` to your function.
(The attribute syntax may be changed in the future, but should be as minimal as changing the name.)

For simplicity of implementation, Rust-Call-Dart uses DCO+SSE and cannot be changed currently,
but this should usually be no problem.
If you find any difficulties due to this, feel free to create an issue.

### Comparison

Firstly, for most use cases, both approaches should be good enough.
But if you want to tune the fast bridge to be even faster for whatever reason, here are some suggestions.

Because of the difference in implementation,
the CST/DCO codec has less memory copies,
thus is especially suitable when you have things like large `Vec<u8>` (`Uint8List`).
On the other hand,
if you are serializing a lot of small objects,
SSE codec may need less heap memory allocation calls,
thus may outperform.

For some benchmarks on the typical cases, which are evaluated continuously on CI,
please refer to [this page](../performance/overview).

## RustOpaque codec

There are currently two codecs underlying the "arbitrary Rust types" features.

* **NOM** (NOrmal Memory): The commonly seen standard one.
* **MOI** (Map Of Ids): Use a HashMap from ids to objects.

The MOI codec mainly helps as an extra test utility.
However, if you are super worried about code safety, this mode may be useful.
It can be enabled by `#[frb(rust_opaque_codec_moi)]` annotation on functions
(this name may be changed, but even if changed, will only be a simple rename).
If needed, there can also be a global flag (feel free create an issue on GitHub).
