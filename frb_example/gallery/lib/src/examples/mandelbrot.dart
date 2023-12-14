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
  var running = false;

  @override
  void initState() {
    super.initState();
  }

  void start() {
    stop();
    running = true;
    () async {
      while (running) {
        final receivedImage = await drawMandelbrot(
          imageSize: const Size(width: 50, height: 50),
          zoomPoint: examplePoint,
          scale: generateScale(),
          numThreads: 4,
        );
        if (mounted) setState(() => image = receivedImage);
      }
    }();
  }

  void stop() {
    running = false;
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Column(
          children: [
            TextButton(onPressed: start, child: const Text('Start')),
            TextButton(onPressed: stop, child: const Text('Stop')),
          ],
        ),
        image != null
            ? SizedBox(
                width: 50,
                height: 50,
                child: Center(
                  child: AnimatedReplaceableImage(
                    image: MemoryImage(image!),
                  ),
                ),
              )
            : Container(),
      ],
    );
  }
}
