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
  var size = 300.0;
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
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  _buildSlider(
                    label: 'Num threads',
                    value: numThreads.toDouble(),
                    onChanged: (newValue) =>
                        setState(() => numThreads = newValue.round()),
                    min: 1,
                    max: 4,
                    divisions: 3,
                  ),
                  _buildSlider(
                    label: 'Image size',
                    value: size,
                    onChanged: (newValue) => setState(() => size = newValue),
                    min: 100,
                    max: 1000,
                  ),
                  const SizedBox(height: 32),
                  Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      TextButton(onPressed: start, child: const Text('Start')),
                      const SizedBox(width: 32),
                      TextButton(onPressed: stop, child: const Text('Stop')),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ),
        Expanded(
          child: Align(
            alignment: Alignment.centerLeft,
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (computeTime != null)
                  Text('Time: ${computeTime!.inMilliseconds}ms'),
                const SizedBox(height: 8),
                SizedBox.square(
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
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildSlider({
    required String label,
    required double value,
    required ValueChanged<double>? onChanged,
    required double min,
    required double max,
    int? divisions,
  }) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        const SizedBox(width: 32),
        SizedBox(
          width: 100,
          child: Text(label),
        ),
        SizedBox(
          width: 200,
          child: Slider(
            value: value,
            onChanged: onChanged,
            min: min,
            max: max,
            divisions: divisions,
            label: value.round().toString(),
            inactiveColor: Colors.blue.shade100,
          ),
        ),
      ],
    );
  }
}
