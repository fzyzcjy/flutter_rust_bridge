import 'package:meta/meta.dart';

@internal
class ExecuteStreamPortGenerator {
  static final _streamSinkNameIndex = <String, int>{};

  static String create(String funcName) {
    final nextIndex = _streamSinkNameIndex.update(funcName, (value) => value + 1, ifAbsent: () => 0);
    return '__frb_streamsink_${funcName}_$nextIndex';
  }
}

@internal
class DropIdPortGenerator {
  static int _nextPort = 0;

  static String create() => '__frb_dart_opaque_drop_${_nextPort++}';
}
