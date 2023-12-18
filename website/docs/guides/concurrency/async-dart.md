# Asynchronous Dart

This library generates functions that are *asynchronous* in Dart by default. So you will see `fn f(..) -> String` becomes `Future<String> f(..)` with that interesting `Future`.

Why? Flutter UI is single-threaded. If you use the intuitive synchronous approach, just like what you will (have to) do with plain-old Flutter bindings, your UI will be *stuck* as long as your Rust code is executing. If your Rust code run for 100ms for a heavy computation, your UI will fully freeze for 100ms and the users will not be happy.

On the other hand, with the generated asynchronous bindings in Dart, you can simply call functions directly in main isolate (thread) of Dart/Flutter, and Rust code will not block the Flutter UI.

Indeed async and `Future`s is almost everywhere in Flutter/Dart, and it has very good built-in support. So no worries about it ;)

Remark: A common mistake is to call Rust code in *another* Dart isolate (i.e. "thread") instead of the main isolate. That is completely not needed, and will only make your life harder. As is described above, even if your Rust code computes for 100ms, the async call will only take, say, 0.1ms, and will not block your UI.

