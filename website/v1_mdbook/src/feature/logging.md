# Logging

Since I have seen some questions asking how logging can be implemented with a Flutter + Rust application, here are some examples.

## Logger in production

In my own app in production, I use the following strategy for Rust logging: Use normal Rust logging methods, such as `info!` and `debug!` macros. The logs are consumed in two places: They are printed via platform-specific methods (like android Logcat and iOS NSLog), and also use a Stream to send them to the Dart side such that my Dart code and further process are using the same pipeline as normal Dart logs (e.g. save to a file, send to server, etc).

The *full* code related to logging in my app can be seen here: [#486](https://github.com/fzyzcjy/flutter_rust_bridge/issues/486).

## Simple logger

Let us implement a simple logging system (adapted from the logging system I use with `flutter_rust_bridge` in my app in production), where Rust code can send logs to Dart code.

The Rust `api.rs`:

```rust,noplayground
pub struct LogEntry {
    pub time_millis: i64,
    pub level: i32,
    pub tag: String,
    pub msg: String,
}

// Simplified just for demonstration.
// To compile, you need a OnceCell, or Mutex, or RwLock
// Also see https://github.com/fzyzcjy/flutter_rust_bridge/issues/398
lazy_static! { static ref log_stream_sink: StreamSink<LogEntry>; }

pub fn create_log_stream(s: StreamSink<LogEntry>) {
    stream_sink = s;
}
```

Now Rust will probably complain at you because `IntoDart` is not implemented for `LogEntry`. This is expected, because `flutter_rust_bridge` will generate this trait implementation for you.
To fix this error you should just rerun `flutter_rust_bridge_codegen`.


Generated Dart code:

```Dart
Stream<LogEntry> createLogStream();
```

Now let us use it in Dart:

```dart
Future<void> setup() async {
    createLogStream().listen((event) {
      print('log from rust: ${event.level} ${event.tag} ${event.msg} ${event.timeMillis}');
    });
}
```

And now we can happily log anything in Rust:

```rust,noplayground
log_stream_sink.add(LogEntry { msg: "hello I am a log from Rust", ... })
```

Of course, you can implement a logger following the Rust's `log` crate wrapping this raw stream sink, then you can use standard Rust logging mechanisms like `info!`. I did exactly that in my project.

### Example: Simple timer

Credits: [this](https://gist.github.com/Desdaemon/be5da0a1c6b4724f20093ef434959744) and [#347](https://github.com/fzyzcjy/flutter_rust_bridge/issues/347).

```rust,noplayground
use anyhow::Result;
use std::{thread::sleep, time::Duration};

use flutter_rust_bridge::StreamSink;

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

And use it in Dart:

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
