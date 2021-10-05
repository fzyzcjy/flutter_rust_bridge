import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/generated_api.dart';
import 'package:flutter_rust_bridge_example/generated_wire.dart';
import 'package:flutter_rust_bridge_example/off_topic_code.dart';

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

void main() => runApp(const MyApp());

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late final dylib =
      Platform.isAndroid ? DynamicLibrary.open('libflutter_rust_bridge_example.so') : DynamicLibrary.process();
  late final api = ExampleApi(ExampleWire(dylib));

  Uint8List? exampleImage;
  String? exampleText;

  @override
  void initState() {
    super.initState();
    runPeriodically(_callExampleFfiOne);
    _callExampleFfiTwo();
  }

  @override
  Widget build(BuildContext context) => buildPageUi(
        exampleImage,
        exampleText,
      );

  Future<void> _callExampleFfiOne() async {
    final receivedImage = await api.drawMandelbrot(
        imageSize: Size(width: 50, height: 50), zoomPoint: examplePoint, scale: generateScale(), numThreads: 4);
    setState(() => exampleImage = receivedImage);
  }

  Future<void> _callExampleFfiTwo() async {
    final receivedText = await api.passingComplexStructs(root: createExampleTree());
    setState(() => exampleText = receivedText);
  }
}
