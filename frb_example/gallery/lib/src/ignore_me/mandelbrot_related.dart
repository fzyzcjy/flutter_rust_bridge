import 'package:flutter/foundation.dart';
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

class MandelbrotPageUI extends StatelessWidget {
  final int numThreads;
  final void Function(int value) setNumThreads;
  final double size;
  final void Function(double value) setSize;
  final Uint8List? image;
  final Duration? computeTime;
  final void Function() start;
  final void Function() stop;

  const MandelbrotPageUI({
    super.key,
    required this.numThreads,
    required this.setNumThreads,
    required this.size,
    required this.setSize,
    required this.image,
    required this.computeTime,
    required this.start,
    required this.stop,
  });

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (_, constraints) => Center(
        child: constraints.maxWidth >= 600
            ? Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Padding(
                    padding: const EdgeInsets.only(right: 64),
                    child: _buildControl(),
                  ),
                  _buildImage(),
                ],
              )
            : Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Padding(
                    padding: const EdgeInsets.only(bottom: 32),
                    child: _buildControl(),
                  ),
                  _buildImage(),
                ],
              ),
      ),
    );
  }

  Widget _buildControl() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        if (kDebugMode)
          Padding(
            padding: const EdgeInsets.only(bottom: 8),
            child: Text(
              '(NOTE: Please use release build for fast Rust)',
              style: TextStyle(color: Colors.orange.shade700, fontSize: 11),
            ),
          ),
        _buildSlider(
          label: 'Num threads',
          value: numThreads.toDouble(),
          onChanged: (newValue) => setNumThreads(newValue.round()),
          min: 1,
          max: 4,
          divisions: 3,
        ),
        _buildSlider(
          label: 'Image size',
          value: size,
          onChanged: setSize,
          min: 100,
          max: 400,
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
    );
  }

  Widget _buildImage() {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        if (computeTime != null) Text('Time: ${computeTime!.inMilliseconds}ms'),
        const SizedBox(height: 8),
        SizedBox.square(
          dimension: size,
          child: image != null
              ? AnimatedReplaceableImage(
                  image: MemoryImage(image!),
                )
              : Material(
                  color: Colors.grey.shade100,
                  child: InkWell(
                    onTap: start,
                    child: const Padding(
                      padding: EdgeInsets.all(16),
                      child: Center(
                        child: Text(
                          'Tap to start',
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
