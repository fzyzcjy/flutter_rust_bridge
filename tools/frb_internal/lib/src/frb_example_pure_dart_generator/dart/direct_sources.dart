import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/builder.dart';
import 'package:recase/recase.dart';

Map<String, String> generateDartDirectSources() {
  return {
    'pseudo_manual/basic_test.dart': _generateBasicRelated(
      postfix: '',
      values: (ty) => ty.interestRawValues,
      valueType: (ty) => ty.dartTypeName,
    ),
    'pseudo_manual/basic_optional_test.dart': _generateBasicRelated(
      postfix: '_optional',
      imports: """
      import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic.dart';
      """,
      values: (ty) => ["null", ...ty.interestRawValues],
      valueType: (ty) => '${ty.dartTypeName}?',
    ),
    'pseudo_manual/basic_list_test.dart': _generateBasicRelated(
      postfix: '_list',
      values: (ty) => [
        ty.listWrapper(ty, ''),
        ...ty.interestRawValues.map((x) => ty.listWrapper(ty, x)),
      ],
      imports: """
      import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
      import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic.dart';
      """,
      enable: (ty) => ty.enableList,
      valueType: (ty) => ty.listName,
    ),
    'pseudo_manual/basic_map_test.dart': _generateBasicRelated(
      postfix: '_map',
      imports: """
      import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic.dart';
      """,
      values: (ty) => ['{}', ...ty.interestRawValues.map((x) => '{42: $x}')],
      valueType: (ty) => 'Map<int, ${ty.dartTypeName}>',
    ),
    '../../benchmark/src/generated.dart': generateBenchmark(),
  };
}

String _generateBasicRelated({
  required String postfix,
  required List<String> Function(BasicTypeInfo) values,
  required String? Function(BasicTypeInfo) valueType,
  String imports = '',
  bool Function(BasicTypeInfo)? enable,
}) {
  final builder = DartFileBuilder(importName: 'basic$postfix');
  builder.imports += '''
  import 'dart:typed_data';
  $imports
  ''';
  for (final ty in kBasicTypes) {
    if (enable?.call(ty) ?? true) {
      builder.addTestsIdentityFunctionCall(
        'exampleBasic${ReCase(postfix).pascalCase}Type${ReCase(ty.name).pascalCase}TwinNormal',
        values(ty),
        valueType: valueType(ty),
      );
    }
  }
  return builder.toString();
}
