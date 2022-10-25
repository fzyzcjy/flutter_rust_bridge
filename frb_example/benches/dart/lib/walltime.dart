import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';

/// simple port of [criterion walltime](https://docs.rs/criterion/0.4.0/criterion/measurement/struct.WallTime.html)
abstract class WallTime {
  double get timeElapsed;
  Unit get unit;

  /// convenience method
  double get timeElapsedMicros;

  /// convenience method
  double get timeElapsedMillis;

  /// start timer
  void start();

  /// stop timer
  void stop();

  /// reset timer
  void reset();

  WallTime();
}
