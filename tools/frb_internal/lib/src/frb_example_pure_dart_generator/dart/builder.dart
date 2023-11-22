import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/preludes.dart';

class DartFileBuilder {
  final String importName;
  String body = '';

  DartFileBuilder({required this.importName});

  @override
  String toString() {
    return '''$kDirectSourcesPrelude
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/$importName.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();
  
  group('$importName', () {
    $body
  });
}
  ''';
  }
}
