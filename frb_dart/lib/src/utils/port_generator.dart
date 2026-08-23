import 'dart:math';

import 'package:meta/meta.dart';

final _contextId = List.generate(
  4,
  (_) => Random.secure().nextInt(1 << 31).toRadixString(16).padLeft(8, '0'),
).join();

/// {@macro flutter_rust_bridge.internal}
@internal
class ExecuteStreamPortGenerator {
  static final _streamSinkNameIndex = <String, int>{};

  /// {@macro flutter_rust_bridge.internal}
  static String create(String funcName) {
    final nextIndex = _streamSinkNameIndex.update(
      funcName,
      (value) => value + 1,
      ifAbsent: () => 0,
    );
    return '__frb_streamsink_${_contextId}_${funcName}_$nextIndex';
  }
}

/// {@macro flutter_rust_bridge.internal}
@internal
class BaseLazyPortIdGenerator {
  static int _nextPort = 0;

  /// {@macro flutter_rust_bridge.internal}
  static String create() => '__frb_lazy_port_${_nextPort++}';
}
