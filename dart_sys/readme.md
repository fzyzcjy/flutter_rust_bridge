# `dart-sys`
#### *Native bindings to the dart native extensions sdk.*

This crate exposes an api for [`dart_api.h`](https://github.com/dart-lang/sdk/blob/master/runtime/include/dart_api.h),
 which exposes the basic [dart](https://dart.dev/)
 native [extensions api](https://dart.dev/server/c-interop-native-extensions). 
 This crate used [`bindgen`](https://github.com/rust-lang/rust-bindgen)
 to generate the bindings to the header.
 
##### Requirements when building bindings again (off by default)
- Provide a path to the dart sdk using a `dart_sdk` environment variable.
  - If this variable is not available, will look for either a chocolatey install
  path, or an entry in the `PATH` variable which contains `dart-sdk` in it.
  This will fall back to the `flutter` sdk should it not find a dart sdk, but this
  is not recommended, as it is more difficult to compile using the flutter sdk
  and it appears it ships a non-standard dart sdk. 

##### Usage
Include the following in your `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]
[dependencies]
dart-sys = "0.1.0"
```
And follow the guide on the [native extensions api page](https://dart.dev/server/c-interop-native-extensions).

##### Examples
Please visit the [examples directory](https://github.com/OptimisticPeach/dart-sys/tree/master/examples) for more information. If there should appear
more idiomatic bindings, I will try to keep this updated to link to them. 

### Note
A few things are not mentioned on the [native extensions api](https://dart.dev/server/c-interop-native-extensions)
page:

- You should compile using an x64 compiler (eg., `[stable|nightly|beta]-x86_64-pc-windows-msvc`)
- You should place the compiled library in the same directory as the root of your dart
package (I.E. outside of your `lib` directory)
- You should make sure these three coincide:
  - The shared object/dynamic link library name.
  - The NAME in `NAME_Init` function when writing a sync extension.
  - The name of the import in `import 'dart-ext:NAME'`.
- When compiling for Linux, name your shared object `libNAME.so`.

>This crate does not generate bindings, and instead uses prebuilt ones. The code to build the bindings
>can be found commented in `build.rs`. 
>
> To run, uncomment and from there, copy the contents of the `bindings.rs` file under
>the directory which is reported in the panic into `lib.rs`.
>
> The bindings were last generated on the 26th of February 2020.

I have manually edited the names under the generated bindings to avoid ugly bindgen names such as
`_Dart_CObject__bindgen_ty_1__bindgen_ty_3`, and instead replaces them with a more appropriate name
based on their usage in the api. (Such as `Dart_NativeString`). 
