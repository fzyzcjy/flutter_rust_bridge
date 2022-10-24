// ignore: unused_import
import 'dart:html';

import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.dart';

class WindowPerformance implements WallTime {
  late double _starts = window.performance.now();

  @override
  void start() {}

  @override
  void stop() {}

  @override
  double get timeElapsed => window.performance.now() - _starts;

  @override
  Unit get unit => Unit.Milliseconds;

  @override
  double get timeElapsedMicros => timeElapsed * 1000;

  @override
  void reset() {
    _starts = window.performance.now();
  }
}
