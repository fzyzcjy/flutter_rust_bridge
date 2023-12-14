import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/rust/api/simple.dart';

Widget buildPageUi(Uint8List? exampleImage) {
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
            (exampleImage != null
                ? SizedBox(
                    width: 50,
                    height: 50,
                    child: Center(
                        child: AnimatedReplaceableImage(
                            image: MemoryImage(exampleImage))))
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

class AnimatedReplaceableImage extends StatefulWidget {
  final ImageProvider image;

  const AnimatedReplaceableImage({Key? key, required this.image})
      : super(key: key);

  @override
  AnimatedReplaceableImageState createState() =>
      AnimatedReplaceableImageState();
}

class AnimatedReplaceableImageState extends State<AnimatedReplaceableImage> {
  ImageProvider? previousImage;

  @override
  void initState() {
    super.initState();
    previousImage = null;
  }

  @override
  void didUpdateWidget(AnimatedReplaceableImage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.image.obtainKey(const ImageConfiguration()) !=
        widget.image.obtainKey(const ImageConfiguration())) {
      previousImage = oldWidget.image;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Image(
      image: widget.image,
      frameBuilder: (BuildContext context, Widget child, int? frame,
              bool wasSynchronouslyLoaded) =>
          (frame == null && previousImage != null)
              ? Stack(children: [Image(image: previousImage!), child])
              : child,
    );
  }
}

const examplePoint = Point(
  x: 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795,
  y: -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262,
);

var _scale = 1.0;

double generateScale() {
  _scale *= 0.5;
  if (_scale < 1e-9) _scale = 1.0;
  return _scale;
}
