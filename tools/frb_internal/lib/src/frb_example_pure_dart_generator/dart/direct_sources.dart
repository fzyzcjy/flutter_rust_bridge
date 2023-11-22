import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/builder.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/direct_sources.dart';
import 'package:recase/recase.dart';

Map<String, String> generateDartDirectSources() {
  return {
    'test/api/primitive_test.dart': _generateTestApiPrimitive(),
    'test/api/optional_primitive_test.dart': _generateTestApiOptionalPrimitive(),
  };
}

String _generateTestApiPrimitive() {
  final builder = DartFileBuilder(importName: 'primitive');
  for (final ty in kPrimitiveTypes) {
    for (final arg in ty.interestValues) {
      builder.body += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await examplePrimitiveType${ReCase(ty.name).pascalCase}(arg: $arg), $arg);
        });
      ''';
    }
  }
  return builder.toString();
}

String _generateTestApiOptionalPrimitive() {
  final builder = DartFileBuilder(importName: 'optional_primitive');
  for (final ty in kPrimitiveTypes) {
    for (final arg in ["null", ...ty.interestValues]) {
      builder.body += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await exampleOptionalPrimitiveType${ReCase(ty.name).pascalCase}(arg: $arg), $arg);
        });
      ''';
    }
  }
  return builder.toString();
}
