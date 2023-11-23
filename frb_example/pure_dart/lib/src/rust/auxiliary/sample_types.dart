// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class MySize {
  final int width;
  final int height;

  const MySize({
    required this.width,
    required this.height,
  });

  @override
  int get hashCode => width.hashCode ^ height.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MySize &&
          runtimeType == other.runtimeType &&
          width == other.width &&
          height == other.height;
}
