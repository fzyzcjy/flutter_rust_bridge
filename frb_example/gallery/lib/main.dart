// ignore_for_file: use_key_in_widget_constructors

import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/rust/api/simple.dart';
import 'package:frb_example_gallery/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge Gallery')),
        body: Center(
          child: Text(
              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
        ),
      ),
    );
  }
}
