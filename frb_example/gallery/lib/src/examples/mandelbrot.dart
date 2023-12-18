import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/ignore_me/mandelbrot_related.dart';
import 'package:frb_example_gallery/src/rust/api/mandelbrot.dart';

class MandelbrotPageBody extends StatefulWidget {
  const MandelbrotPageBody({super.key});

  @override
  State<MandelbrotPageBody> createState() => _MandelbrotPageBodyState();
}

class _MandelbrotPageBodyState extends State<MandelbrotPageBody> {
  Uint8List? image;
  Duration? computeTime;
  SimpleRunner? runner;
  var size = 150.0;
  var numThreads = 1;

  @override
  void dispose() {
    stop();
    super.dispose();
  }

  void start() {
    stop();
    runner =
        SimpleRunner(minDuration: const Duration(milliseconds: 33), () async {
      final watch = Stopwatch()..start();
      final effectiveSize = (size.round() ~/ numThreads) * numThreads;
      final receivedImage = await drawMandelbrot(
        imageSize: Size(width: effectiveSize, height: effectiveSize),
        zoomPoint: examplePoint,
        scale: generateScale(),
        numThreads: numThreads,
      );
      watch.stop();

      if (mounted) {
        setState(() {
          image = receivedImage;
          computeTime = watch.elapsed;
        });
      }
    });
  }

  void stop() {
    runner?.dispose();
    runner = null;
  }

  @override
  Widget build(BuildContext context) {
    // Since we are not providing a demo to teach Flutter itself,
    // how the detailed UI is constructed is unrelated
    return MandelbrotPageUI(
      numThreads: numThreads,
      setNumThreads: (value) => setState(() => numThreads = value),
      size: size,
      setSize: (value) => setState(() => size = value),
      image: image,
      computeTime: computeTime,
      start: start,
      stop: stop,
    );
  }
}
