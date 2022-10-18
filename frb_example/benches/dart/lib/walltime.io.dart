import 'package:flutter_rust_bridge_benchmark/bridge_definitions.dart';
import 'package:flutter_rust_bridge_benchmark/walltime.dart';

class AsyncStopWatch extends Stopwatch implements WallTime {
  @override
  double get timeElapsed => elapsedMicroseconds.toDouble();

  @override
  double get timeElapsedMicros => timeElapsed;

  @override
  Unit get unit => Unit.Microseconds;
}
