import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/rust/api/mandelbrot.dart';

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
      fit: BoxFit.fill,
      frameBuilder: (BuildContext context, Widget child, int? frame,
              bool wasSynchronouslyLoaded) =>
          (frame == null && previousImage != null)
              ? Stack(
                  fit: StackFit.passthrough,
                  children: [
                    Image(
                      image: previousImage!,
                      fit: BoxFit.fill,
                    ),
                    child,
                  ],
                )
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
  _scale *= 0.95;
  if (_scale < 1e-9) _scale = 1.0;
  return _scale;
}

class SimpleRunner {
  var _running = true;

  SimpleRunner(Future<void> Function() run, {required Duration minDuration}) {
    () async {
      while (_running) {
        final watch = Stopwatch()..start();
        await run();
        watch.stop();
        final actualDuration = watch.elapsed;
        if (minDuration > actualDuration) {
          await Future.delayed(minDuration - actualDuration);
        }
      }
    }();
  }

  void dispose() {
    _running = false;
  }
}
