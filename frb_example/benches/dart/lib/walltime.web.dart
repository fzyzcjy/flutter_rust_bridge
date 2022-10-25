// ignore: unused_import
import 'dart:html';

import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.dart';

/// use milliseconds on Dart web
class WindowPerformance implements WallTime {
  late double _starts = window.performance.now();

  @override
  void start() {}

  @override
  void stop() {}

  @override
  double get timeElapsed => _starts - window.performance.now();

  @override
  Unit get unit => Unit.Milliseconds;

  @override
  double get timeElapsedMicros => timeElapsed * 1000;

  @override
  double get timeElapsedMillis => timeElapsed;

  @override
  void reset() {
    _starts = window.performance.now();
  }
}
