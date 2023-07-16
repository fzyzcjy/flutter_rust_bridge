# `Stream` / `Iterator`

What is `Stream`? In short: call once, return multiple times; like `Iterator`s.

Flutter's [Stream](https://dart.dev/tutorials/language/streams) is a powerful abstraction. When using it as the return value of Rust function, we can allow the scenario that we call function once, and then return multiple times.

For example, your Rust function may run computationally heavy algorithms, and for every hundreds of milliseconds, it finds out a new piece of the full solution. In this case, it can immediately give that piece to Flutter, then Flutter can render it to UI immediately. Therefore, users do not need to wait for the full algorithm to finish before he can see some partial results on the user interface.

As for the details, a Rust function with signature like `fn f(sink: StreamSink<T>, ..) -> Result<()>` is translated to a Dart function `Stream<T> f(..)`.

Notice that, you can hold that `StreamSink` forever, and use it freely even _after the Rust function itself returns_. The logger example below also demonstrates this (the `create_log_stream` returns almost immediately, while you can use the `StreamSink` after, say, an hour).

The `StreamSink` can be placed at any location. For example, `fn f(a: i32, b: StreamSink<String>)` and `fn f(a: StreamSink<String>, b: i32)` are both valid.

## Examples

See [logging examples](logging.md) which uses streams extensively.

## Mirrored types in streams

For mirrored types a Trait is generated that is used to implement the add method on the StreamSink. This trait is called `<T>StreamSink` where T is the type of the mirrored type. For example, if you have a mirrored type `MyType` then the trait is called `MyTypeStreamSink`. This trait has to be imported to be able to use a `StreamSink<MyType>`.
The usage of the Sink doesn't change.

Example:

```rust,noplayground
...
pub use external_lib::MyType;
use MyTypeStreamSink;

#[frb(mirror(MyType))]
_MyType{
    ...
}

pub fn my_type_stream_sink_example(sink: StreamSink<MyType>) {
    sink.add(MyType::new());
}

```

## What about streaming from Dart/Flutter to Rust?

This is not currently supported. As a workaround, consider iterating through your Dart stream and calling a normal Rust function for each item.
