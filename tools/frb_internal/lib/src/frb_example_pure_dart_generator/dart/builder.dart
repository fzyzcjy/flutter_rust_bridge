import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/preludes.dart';

class DartFileBuilder {
  final String importName;
  String imports = '';
  String body = '';

  DartFileBuilder({required this.importName});

  void addTestsIdentityFunctionCall(
    String funcName,
    List<String> values, {
    String? valueType,
  }) {
    if (values.isEmpty) throw ArgumentError();
    final bracketedValueType = valueType == null ? "" : '<$valueType>';
    body +=
        'addTestsIdentityFunctionCall($funcName, $bracketedValueType[${values.join(", ")}]);';
  }

  @override
  String toString() {
    return '''$kDirectSourcesPrelude
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/$importName.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
$imports

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();
  
  group('$importName', () {
    $body
  });
}
  ''';
  }
}
