import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/builder.dart';
import 'package:recase/recase.dart';

Map<String, String> generateDartDirectSources() {
  return {
    'pseudo_manual/basic_test.dart': _generateBasic(),
    'pseudo_manual/optional_basic_test.dart': _generateOptionalBasic(),
    'pseudo_manual/basic_list_test.dart': _generateBasicList(),
    'pseudo_manual/basic_map_test.dart': _generateBasicMap(),
    '../../benchmark/src/generated.dart': generateBenchmark(),
  };
}

String _generateBasic() {
  final builder = DartFileBuilder(importName: 'basic');
  for (final ty in kBasicTypes) {
    final values = ty.interestRawValues
        .map((value) => ty.primitiveWrapper(ty, value))
        .toList();
    builder.addTestsIdentityFunctionCall(
        'exampleBasicType${ReCase(ty.name).pascalCase}TwinNormal', values,
        valueType: ty.dartTypeName);
  }
  return builder.toString();
}

String _generateBasicList() {
  final builder = DartFileBuilder(importName: 'basic_list');

  builder.imports += """
  import 'dart:typed_data';
  import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
  """;

  for (final ty in kBasicTypes) {
    final values = [
      ty.primitiveListWrapper(ty, ''),
      ...ty.interestRawValues.map((x) => ty.primitiveListWrapper(ty, x)),
    ];
    builder.addTestsIdentityFunctionCall(
        'exampleBasicListType${ReCase(ty.name).pascalCase}TwinNormal', values);
  }
  return builder.toString();
}

String _generateBasicMap() {
  throw UnimplementedError();
}

String _generateOptionalBasic() {
  final builder = DartFileBuilder(importName: 'optional_basic');
  for (final ty in kBasicTypes) {
    final values = [
      "null",
      ...ty.interestRawValues.map((x) => ty.primitiveWrapper(ty, x)),
    ];
    builder.addTestsIdentityFunctionCall(
        'exampleOptionalBasicType${ReCase(ty.name).pascalCase}TwinNormal',
        values,
        valueType: '${ty.dartTypeName}?');
  }
  return builder.toString();
}
