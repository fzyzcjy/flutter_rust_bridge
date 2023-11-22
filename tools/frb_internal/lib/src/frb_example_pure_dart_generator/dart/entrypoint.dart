import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/entrypoint.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';
import 'package:recase/recase.dart';

Future<void> generateDart({required Uri dartRoot}) async {
  final textOfPathMap = {
    'test/api/primitive_test.dart': _generateTestApiPrimitive(),
    'test/api/optional_primitive_test.dart': _generateTestApiOptionalPrimitive(),
  };

  writeCodeFiles(dartRoot, textOfPathMap);
  await executeDartFormat(workingDirectory: dartRoot.toFilePath());
}

String _generateTestApiPrimitive() {
  var ans = '';
  for (final ty in kPrimitiveTypes) {
    for (final arg in ty.interestValues) {
      ans += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await examplePrimitiveType${ReCase(ty.name).pascalCase}(arg: $arg), $arg);
        });
      ''';
    }
  }
  return _generateTestTemplate(ans, importName: 'primitive');
}

String _generateTestApiOptionalPrimitive() {
  var ans = '';
  for (final ty in kPrimitiveTypes) {
    for (final arg in ["null", ...ty.interestValues]) {
      ans += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await exampleOptionalPrimitiveType${ReCase(ty.name).pascalCase}(arg: $arg), $arg);
        });
      ''';
    }
  }
  return _generateTestTemplate(ans, importName: 'optional_primitive');
}

String _generateTestTemplate(String body, {required String importName}) {
  return '''
import 'package:frb_example_pure_dart/src/rust/api/$importName.dart';
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
