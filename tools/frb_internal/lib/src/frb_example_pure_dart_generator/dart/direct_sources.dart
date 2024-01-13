import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/builder.dart';
import 'package:recase/recase.dart';

Map<String, String> generateDartDirectSources() {
  return {
    'pseudo_manual/basic_test.dart': _generateBasicRelated(
      postfix: '',
      values: (ty) => ty.interestRawValues
          .map((value) => ty.primitiveWrapper(ty, value))
          .toList(),
      valueType: (ty) => ty.dartTypeName,
    ),
    'pseudo_manual/basic_optional_test.dart': _generateBasicRelated(
      postfix: '_optional',
      values: (ty) => [
        "null",
        ...ty.interestRawValues.map((x) => ty.primitiveWrapper(ty, x)),
      ],
      valueType: (ty) => '${ty.dartTypeName}?',
    ),
    'pseudo_manual/basic_list_test.dart': _generateBasicRelated(
      postfix: '_list',
      values: (ty) => [
        ty.primitiveListWrapper(ty, ''),
        ...ty.interestRawValues.map((x) => ty.primitiveListWrapper(ty, x)),
      ],
      imports: """
      import 'dart:typed_data';
      import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
      """,
      valueType: (ty) => null,
    ),
    'pseudo_manual/basic_map_test.dart': _generateBasicRelated(
      postfix: '_map',
      values: (ty) => [
        ...ty.interestRawValues, // TODO
      ],
      valueType: (ty) => null,
    ),
    '../../benchmark/src/generated.dart': generateBenchmark(),
  };
}

String _generateBasicRelated({
  required String postfix,
  required List<String> Function(BasicTypeInfo) values,
  required String? Function(BasicTypeInfo) valueType,
  String imports = '',
}) {
  final builder = DartFileBuilder(importName: 'basic$postfix');
  builder.imports = imports;
  for (final ty in kBasicTypes) {
    builder.addTestsIdentityFunctionCall(
      'exampleBasic${ReCase(postfix).pascalCase}Type${ReCase(ty.name).pascalCase}TwinNormal',
      values(ty),
      valueType: valueType(ty),
    );
  }
  return builder.toString();
}
