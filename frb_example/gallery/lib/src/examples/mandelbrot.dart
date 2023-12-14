import 'dart:async';
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
  Uint8List? image;
  Timer? runningTimer;

  @override
  void initState() {
    super.initState();
  }

  void start() {
    stop();
    runningTimer = Timer.periodic(const Duration(milliseconds: 500), (_) async {
      final receivedImage = await drawMandelbrot(
        imageSize: const Size(width: 50, height: 50),
        zoomPoint: examplePoint,
        scale: generateScale(),
        numThreads: 4,
      );
      if (mounted) setState(() => image = receivedImage);
    });
  }

  void stop() {
    runningTimer?.cancel();
    runningTimer = null;
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 24),
      child: Card(
        child: Container(
          padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
          child: Column(
            children: [
              const Text('Example 1',
                  style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
              Container(height: 8),
              const Text(
                  'Image generated (periodically) by Rust and displayed by Flutter/Dart'),
              Container(height: 24),
              (image != null
                  ? SizedBox(
                      width: 50,
                      height: 50,
                      child: Center(
                          child: AnimatedReplaceableImage(
                              image: MemoryImage(image))))
                  : Container()),
              Container(height: 4),
              const Text('Mandelbrot Set',
                  style: TextStyle(fontSize: 11, color: Colors.grey)),
              const Text('classical image requiring lots of computing',
                  style: TextStyle(fontSize: 11, color: Colors.grey)),
              Container(height: 8),
            ],
          ),
        ),
      ),
    );
  }
}
