import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/examples/mandelbrot.dart';
import 'package:frb_example_gallery/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          surface: Colors.white,
          primary: Colors.blue,
        ),
      ),
      home: const Scaffold(
        body: MandelbrotPageBody(),
      ),
    );
  }
}
