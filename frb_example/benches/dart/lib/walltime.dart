import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';

/// timer
abstract class WallTime {
  double get timeElapsed;
  Unit get unit;

  /// convenience method
  double get timeElapsedMicros;

  /// start timer
  void start();

  /// stop timer
  void stop();

  /// reset timer
  void reset();

  WallTime();
}
