import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/preludes.dart';

class RustFileBuilder {
  String body = '';

  void addIdentityFunction(String ty, String partialName) {
    body += 'pub fn ${partialName}_twin_normal(arg: $ty) -> $ty { arg }\n\n';
  }

  @override
  String toString() {
    return '''$kDirectSourcesPrelude
$body
''';
  }
}
