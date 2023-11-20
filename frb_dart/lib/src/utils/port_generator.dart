class _ExecuteStreamPortGenerator {
  static final _streamSinkNameIndex = <String, int>{};

  static String _nextName(String funcName) {
    final nextIndex = _streamSinkNameIndex.update(funcName, (value) => value + 1, ifAbsent: () => 0);
    return '__frb_streamsink_${funcName}_$nextIndex';
  }
}

class _DropIdPortGenerator {
  static final instance = _DropIdPortGenerator._();

  _DropIdPortGenerator._();

  int _nextPort = 0;

  String create() => '__frb_dart_opaque_drop_${_nextPort++}';
}
