import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/material.dart' hide Size;
import 'bridge_definitions.dart';
import 'package:flutter_rust_bridge_example/off_topic_code.dart';

import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
export 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart' show api;

// Simple Flutter code. If you are not familiar with Flutter, this may sounds a bit long. But indeed
// it is quite trivial and Flutter is just like that. Please refer to Flutter's tutorial to learn Flutter.

void main() => runApp(const MyApp());

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
    _callExampleFfiChrono();
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

Future<void> _callExampleFfiChrono() async {
  const duration = Duration(hours: 4);
  final naive = DateTime.utc(2022, 09, 10, 20, 48, 53, 123, 456);
  final local = DateTime.now();
  final utc = DateTime.now().toUtc();
  final difference =
      await api.howLongDoesItTake(mine: FeatureChrono(utc: utc, local: local, duration: duration, naive: naive));
  debugPrint('$difference');
}
