# `Stream` / `Iterator`

What is `Stream`? In short: call once, return multiple times; like `Iterator`s.

Flutter's [Stream](https://dart.dev/tutorials/language/streams) is a powerful abstraction. When using it as the return value of Rust function, we can allow the scenario that we call function once, and then return multiple times.

For example, your Rust function may run computationally heavy algorithms, and for every hundreds of milliseconds, it finds out a new piece of the full solution. In this case, it can immediately give that piece to Flutter, then Flutter can render it to UI immediately. Therefore, users do not need to wait for the full algorithm to finish before he can see some partial results on the user interface.

As for the details, a Rust function with signature like `fn f(sink: StreamSink<T>, ..) -> Result<()>` is translated to a Dart function `Stream<T> f(..)`.

Notice that, you can hold that `StreamSink` forever, and use it freely even _after the Rust function itself returns_. The logger example below also demonstrates this (the `create_log_stream` returns almost immediately, while you can use the `StreamSink` after, say, an hour).

The `StreamSink` can be placed at any location. For example, `fn f(a: i32, b: StreamSink<String>)` and `fn f(a: StreamSink<String>, b: i32)` are both valid.

## Examples

See [logging examples](logging.md) which uses streams extensively.

## What about streaming from Dart/Flutter to Rust?

This is not currently supported. As a workaround, consider iterating through your Dart stream and calling a normal Rust function for each item.

## Streaming external types (mirror)

When using mirrored external types in a StreamSink you have to specify the generated `mirror_..` type in the StreamSink. This is because the generated code can not implement the necessary interfaces on a struct from a different crate. The generated `mirror_struct` implements `From<struct>` so you can use your normal types and just call `.into()` before passing them to the StreamSink.

### For example:

```rust,noplayground
pub use external_crate::MyStruct;

pub fn sink_function(sink: StreamSink<mirror_MyStruct>) {
    sink.add(MyStruct::new().into());
}

// for vectors you have to call mirror_struct::from(vec) instead of vec.into()
pub fn vec_sink_function(sink: StreamSink<Vec<mirror_MyStruct>>>){
    let vector = vec![MyStruct::new(), MyStruct::new()];
    sink.add(mirror_MyStruct::from(vector));
}

// mirror types in structs work directly
#[derive(Default)]
pub struct MirrorStruct {
    pub my_struct: mirror_MyStruct,
}

pub fn struct_sink_function(sink: StreamSink<MyStructWrapper>) {
    let mirror_struct = MyStruct::default();
    sink.add(MyStructWrapper {
        my_struct: MyStruct::new().into(),
    });
}
```
