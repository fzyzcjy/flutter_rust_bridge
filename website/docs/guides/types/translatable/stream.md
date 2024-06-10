# Stream / Iterator

## Introduction

What is `Stream`? In short: call once, return multiple times; like `Iterator`s.

Flutter's [Stream](https://dart.dev/tutorials/language/streams) is a powerful abstraction. When using it as the return
value of Rust function, we can allow the scenario that we call function once, and then return multiple times.

For example, your Rust function may run computationally heavy algorithms, and for every hundreds of milliseconds, it
finds out a new piece of the full solution. In this case, it can immediately give that piece to Flutter, then Flutter
can render it to UI immediately. Therefore, users do not need to wait for the full algorithm to finish before he can see
some partial results on the user interface.

## Remarks

As for the details, a Rust function with signature like `fn f(sink: StreamSink<T>, ..) -> Result<()>` is translated to a
Dart function `Stream<T> f(..)`.

Notice that, you can hold that `StreamSink` forever, and use it freely even *after the Rust function itself returns*.
The logger example below also demonstrates this (the `create_log_stream` returns almost immediately, while you can use
the `StreamSink` after, say, an hour).

The `StreamSink` can be placed at any location. For example, `fn f(a: i32, b: StreamSink<String>)`
and `fn f(a: StreamSink<String>, b: i32)` are both valid.

## Stream argument in arbitrary types

The `StreamSink` type can also be placed in arbitrary types, such as inside structs, enums, vectors, ...
See example 3 below for more details.

## Control whether to wait

By default, the stream is immediately usable in Dart, and the Rust function is *not* awaited.

If you need the Rust function to finish execution *before* the stream can be obtained in Dart,
just add `#[frb(stream_dart_await)]` (to await the stream) or `#[frb(sync)]` (to let the whole function be synchronous) to
the function.

## Add an error

To put an error into the stream, the `stream.add_error` method can be utilized.
It currently accepts an `anyhow::Error` type.

For example, we can write down `stream.add_error(anyhow::anyhow!("hello"))` and the Dart side will see an exception thrown.

## Examples

See [logging examples](../../how-to/logging) which uses streams extensively.

### Simple

Simply iterate through your Dart stream, and call a normal Rust function for each item.
For example:

```dart
myStream.listen((data) => myRustfunction(data));
```

While on the Rust side:

```rust
fn my_rust_function(data: WhateverType) { ... }
```

### A timer

Credits: [this](https://gist.github.com/Desdaemon/be5da0a1c6b4724f20093ef434959744)
and [#347](https://github.com/fzyzcjy/flutter_rust_bridge/issues/347).

<details>

```rust
use anyhow::Result;
use std::{thread::sleep, time::Duration};

use crate::frb_generated::StreamSink;

const ONE_SECOND: Duration = Duration::from_secs(1);

// can't omit the return type yet, this is a bug
pub fn tick(sink: StreamSink<i32>) -> Result<()> {
    let mut ticks = 0;
    loop {
        sink.add(ticks);
        sleep(ONE_SECOND);
        if ticks == i32::MAX {
            break;
        }
        ticks += 1;
    }
    Ok(())
}
```

</details>

And use it in Dart:

<details>

```dart
import 'package:flutter/material.dart';
import 'ffi.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  late Stream<int> ticks;

  @override
  void initState() {
    super.initState();
    ticks = api.tick();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text("Time since starting Rust stream"),
            StreamBuilder<int>(
              stream: ticks,
              builder: (context, snap) {
                final style = Theme.of(context).textTheme.headlineMedium;
                final error = snap.error;
                if (error != null)
                  return Tooltip(
                      message: error.toString(),
                      child: Text('Error', style: style));

                final data = snap.data;
                if (data != null) return Text('$data second(s)', style: style);

                return const CircularProgressIndicator();
              },
            )
          ],
        ),
      ),
    );
  }
}
```

</details>

### Stream type inside arbitrary types

For example, we can place it as a field of a struct or inside a vector:

```rust
pub struct MyStruct {
    a: String,
    b: StreamSink<i32>,
}

pub fn f(arr: Vec<StreamSink<i32>>, st: MyStruct) {}
```
