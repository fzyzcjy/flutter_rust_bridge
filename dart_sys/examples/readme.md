## Examples

This includes both of the examples given on the [dart native extensions page](https://dart.dev/server/c-interop-native-extensions),
the synchronous example is in `./sync.rs`, and the asynchronous example is in `./async.rs`.


To run them, copy them into another project, which will build as a `cdylib`, so as to
not need to have the `main` function present in them. Then, with the compiled library, 
copy it into your dart package's root and use a `dart-ext:mylib` import to import it
into your package. 
