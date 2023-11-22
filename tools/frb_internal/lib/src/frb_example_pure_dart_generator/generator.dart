import 'dart:io';

void generate() {
  final dartRoot = Directory.current.uri.resolve('../../frb_example/pure_dart');
  if (!File(dartRoot.toFilePath()).existsSync()) throw StateError('dartRoot=$dartRoot does not exist');

  // TODO
}
