import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/builder.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/direct_sources.dart';
import 'package:recase/recase.dart';

Map<String, String> generateDartDirectSources() {
  return {
    'pseudo_manual/primitive_test.dart': _generatePrimitive(),
    'pseudo_manual/optional_primitive_test.dart': _generateOptionalPrimitive(),
    'pseudo_manual/primitive_list_test.dart': _generatePrimitiveList(),
  };
}

String _generatePrimitive() {
  final builder = DartFileBuilder(importName: 'primitive');
  for (final ty in kPrimitiveTypes) {
    for (final arg in ty.interestValues) {
      builder.body += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await examplePrimitiveType${ReCase(ty.name).pascalCase}TwinNormal(arg: $arg), $arg);
        });
      ''';
    }
  }
  return builder.toString();
}

String _generatePrimitiveList() {
  final builder = DartFileBuilder(importName: 'primitive_list');
  for (final ty in kPrimitiveTypes) {
    for (final arg in [
      '[]',
      ...ty.interestValues.map((x) => '[$x]'),
    ]) {
      builder.body += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await examplePrimitiveListType${ReCase(ty.name).pascalCase}TwinNormal(arg: $arg), $arg);
        });
      ''';
    }
  }
  return builder.toString();
}

String _generateOptionalPrimitive() {
  final builder = DartFileBuilder(importName: 'optional_primitive');
  for (final ty in kPrimitiveTypes) {
    for (final arg in ["null", ...ty.interestValues]) {
      builder.body += '''
        test('type=${ty.name} arg=$arg', () async {
          expect(await exampleOptionalPrimitiveType${ReCase(ty.name).pascalCase}TwinNormal(arg: $arg), $arg);
        });
      ''';
    }
  }
  return builder.toString();
}
