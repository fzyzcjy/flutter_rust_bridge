import 'dart:typed_data';

import 'package:flutter/material.dart';

class MandelbrotPageBody extends StatefulWidget {
  const MandelbrotPageBody({super.key});

  @override
  State<MandelbrotPageBody> createState() => _MandelbrotPageBodyState();
}

class _MandelbrotPageBodyState extends State<MandelbrotPageBody> {
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
        imageSize: const Size(width: 50, height: 50),
        zoomPoint: examplePoint,
        scale: generateScale(),
        numThreads: 4);
    if (mounted) setState(() => exampleImage = receivedImage);
  }

  Future<void> _callExampleFfiTwo() async {
    final receivedText =
        await api.passingComplexStructs(root: createExampleTree());
    if (mounted) setState(() => exampleText = receivedText);
  }
}
