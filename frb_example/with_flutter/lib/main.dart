import 'dart:async';
import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/bridge_generated.dart';
import 'package:flutter_rust_bridge_example/off_topic_code.dart';

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

late final dylib =
    Platform.isAndroid ? DynamicLibrary.open('libflutter_rust_bridge_example.so') : DynamicLibrary.process();
late final api = FlutterRustBridgeExample(dylib);

void main() => runApp(const MyExprApp());

class MyExprApp extends StatefulWidget {
  const MyExprApp({Key? key}) : super(key: key);

  @override
  _MyExprAppState createState() => _MyExprAppState();
}

class _MyExprAppState extends State<MyExprApp> {
  @override
  void initState() {
    super.initState();
    _execute();
  }

  Future<void> _execute() async {
    await _debugThrow('RETURN_ERR');
    await _debugThrow('PANIC');
  }

  Future<void> _debugThrow(String mode) async {
    print('debugThrow $mode');
    try {
      await api.offTopicDebugThrow(mode: mode);
    } catch (e, s) {
      debugPrint('catch error for debugThrow e=$e s=$s');
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(),
        body: const Text('hi'),
      ),
    );
  }
}

class MyApp extends StatefulWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
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
    if (mounted) setState(() => exampleImage = receivedImage);
  }

  Future<void> _callExampleFfiTwo() async {
    final receivedText = await api.passingComplexStructs(root: createExampleTree());
    if (mounted) setState(() => exampleText = receivedText);
  }
}
