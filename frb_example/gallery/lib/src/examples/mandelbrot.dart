import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/ignore_me/mandelbrot_related.dart';
import 'package:frb_example_gallery/src/rust/api/simple.dart';

class MandelbrotPageBody extends StatefulWidget {
  const MandelbrotPageBody({super.key});

  @override
  State<MandelbrotPageBody> createState() => _MandelbrotPageBodyState();
}

class _MandelbrotPageBodyState extends State<MandelbrotPageBody> {
  Uint8List? exampleImage;

  @override
  void initState() {
    super.initState();
    runPeriodically(_callExampleFfiOne);
  }

  @override
  Widget build(BuildContext context) => buildPageUi(exampleImage);

  Future<void> _callExampleFfiOne() async {
    final receivedImage = await drawMandelbrot(
      imageSize: const Size(width: 50, height: 50),
      zoomPoint: examplePoint,
      scale: generateScale(),
      numThreads: 4,
    );
    if (mounted) setState(() => exampleImage = receivedImage);
  }
}
