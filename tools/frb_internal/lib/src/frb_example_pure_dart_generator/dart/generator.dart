import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/generator.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';
import 'package:recase/recase.dart';

Future<void> generateDart({required Uri dartRoot}) async {
  final textOfPathMap = {
    'test/api/primitive_test.dart': _generateTestApiPrimitive(),
  };

  writeCodeFiles(dartRoot, textOfPathMap);
  await executeDartFormat(workingDirectory: dartRoot.toFilePath());
}

String _generateTestApiPrimitive() {
  var ans = '''
import 'package:frb_example_pure_dart/src/rust/api/primitive.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();
  ''';

  for (final ty in kPrimitiveTypes) {
    ans += '''
      test('primitive type $ty as argument and return type', () async {
        expect(await examplePrimitiveType${ReCase(ty).pascalCase}(arg: 100), 100);
      });
    ''';
  }

  ans += '}';
  return ans;
}
