import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/preludes.dart';

class RustFileBuilder {
  String body = '';

  @override
  String toString() {
    return '''$kDirectSourcesPrelude
$body
''';
  }
}
