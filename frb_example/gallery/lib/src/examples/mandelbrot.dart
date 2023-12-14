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
  var size = 200.0;
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
    return Row(
      children: [
        Expanded(
          child: Align(
            alignment: Alignment.centerRight,
            child: Padding(
              padding: const EdgeInsets.only(right: 64),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  for (final candidateNumThreads in [1, 2, 4])
                    TextButton(
                      onPressed: () {
                        numThreads = candidateNumThreads;
                        start();
                      },
                      child: Text('Start ($candidateNumThreads threads)'),
                    ),
                  TextButton(onPressed: stop, child: const Text('Stop')),
                  SizedBox(
                    width: 200,
                    child: Slider(
                      value: size,
                      onChanged: (newValue) => setState(() => size = newValue),
                      min: 100,
                      max: 1000,
                      inactiveColor: Colors.blue.shade100,
                    ),
                  ),
                  if (computeTime != null)
                    SizedBox(
                      width: 128,
                      child: Text(
                          'Compute time: ${computeTime!.inMilliseconds}ms'),
                    ),
                ],
              ),
            ),
          ),
        ),
        Expanded(
          child: Align(
            alignment: Alignment.centerLeft,
            child: SizedBox.square(
              dimension: size,
              child: image != null
                  ? AnimatedReplaceableImage(
                      image: MemoryImage(image!),
                    )
                  : Container(
                      color: Colors.grey.shade100,
                      padding: const EdgeInsets.all(16),
                      child: const Center(
                        child: Text(
                          'Use buttons on the left to start animation',
                          style: TextStyle(color: Colors.grey),
                          textAlign: TextAlign.center,
                        ),
                      ),
                    ),
            ),
          ),
        ),
      ],
    );
  }
}
