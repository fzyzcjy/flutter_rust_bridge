/// Code in this file is more about Flutter than our package, `flutter rust bridge`.
/// To understand this package, you do not need to have a deep understanding of this file and Flutter.
/// Thus, we put it in this "utility" file, instead of the "main" file.
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge_example/generated_api.dart';

Widget buildCardUi(String title, String subtitle, Widget child) {
  return Container(
    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
    child: Card(
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
        child: Column(
          children: [
            Text(title, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
            Container(height: 8),
            Text(subtitle),
            child,
          ],
        ),
      ),
    ),
  );
}

TreeNode createExampleTree() => TreeNode(name: 'root', children: [
      for (var i = 0; i < 1 + Random().nextInt(2); ++i)
        TreeNode(name: 'child_$i', children: [
          for (var j = 0; j < 1 + Random().nextInt(2); ++j) TreeNode(name: 'grandchild_$j', children: [])
        ]),
    ]);

final examplePoint = Point(
    x: 0.3602404434376143632361252444495453084826078079585857504883758147401953460592181003117529367227734263962337317297249877373200353726832853176645324012185215795,
    y: -0.6413130610648031748603750151793020665794949522823052595561775430644485741727536902556370230689681162370740565537072149790106973211105273740851993394803287437606238596262);

class AnimatedReplaceableImage extends StatefulWidget {
  final ImageProvider image;

  const AnimatedReplaceableImage({Key? key, required this.image}) : super(key: key);

  @override
  _AnimatedReplaceableImageState createState() => _AnimatedReplaceableImageState();
}

class _AnimatedReplaceableImageState extends State<AnimatedReplaceableImage> {
  ImageProvider? previousImage;

  @override
  void initState() {
    super.initState();
    previousImage = null;
  }

  @override
  void didUpdateWidget(AnimatedReplaceableImage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.image.obtainKey(const ImageConfiguration()) != widget.image.obtainKey(const ImageConfiguration())) {
      previousImage = oldWidget.image;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Image(
      image: widget.image,
      frameBuilder: (BuildContext context, Widget child, int? frame, bool wasSynchronouslyLoaded) =>
          (frame == null && previousImage != null) ? Stack(children: [Image(image: previousImage!), child]) : child,
    );
  }
}
