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
    final values = ty.interestRawValues
        .map((value) => ty.primitiveWrapper(ty, value))
        .toList();
    builder.addTestsIdentityFunctionCall(
        'examplePrimitiveType${ReCase(ty.name).pascalCase}TwinNormal', values,
        valueType: ty.dartTypeName);
  }
  return builder.toString();
}

String _generatePrimitiveList() {
  final builder = DartFileBuilder(importName: 'primitive_list');

  builder.imports += """
  import 'dart:typed_data';
  import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
  """;

  for (final ty in kPrimitiveTypes) {
    final values = [
      ty.primitiveListWrapper(ty, ''),
      ...ty.interestRawValues.map((x) => ty.primitiveListWrapper(ty, x)),
    ];
    builder.addTestsIdentityFunctionCall(
        'examplePrimitiveListType${ReCase(ty.name).pascalCase}TwinNormal',
        values);
  }
  return builder.toString();
}

String _generateOptionalPrimitive() {
  final builder = DartFileBuilder(importName: 'optional_primitive');
  for (final ty in kPrimitiveTypes) {
    final values = [
      "null",
      ...ty.interestRawValues.map((x) => ty.primitiveWrapper(ty, x)),
    ];
    builder.addTestsIdentityFunctionCall(
        'exampleOptionalPrimitiveType${ReCase(ty.name).pascalCase}TwinNormal',
        values,
        valueType: '${ty.dartTypeName}?');
  }
  return builder.toString();
}
