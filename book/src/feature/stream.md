# `Stream` / `Iterator`

What is `Stream`? In short: call once, return multiple times; like `Iterator`s.

Flutter's [Stream](https://dart.dev/tutorials/language/streams) is a powerful abstraction. When using it as the return value of Rust function, we can allow the scenario that we call function once, and then return multiple times.

For example, your Rust function may run computationally heavy algorithms, and for every hundreds of milliseconds, it finds out a new piece of the full solution. In this case, it can immediately give that piece to Flutter, then Flutter can render it to UI immediately. Therefore, users do not need to wait for the full algorithm to finish before he can see some partial results on the user interface.

As for the details, a Rust function with signature like `fn f(sink: StreamSink<T>, ..) -> Result<()>` is translated to a Dart function  `Stream<T> f(..)`.