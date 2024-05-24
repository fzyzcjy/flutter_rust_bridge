import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/dart/builder.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/utils/generator_utils.dart';
import 'package:recase/recase.dart';

Map<String, String> generateDartDirectSources(Package package) {
  return {
    'pseudo_manual/basic_test.dart': _generateBasicRelated(
      package,
      postfix: '',
      values: (ty) => ty.interestRawValues.map((x) => x.textAndGuard).toList(),
      valueType: (ty) => ty.dartTypeName,
      withExpect: (ty) => ty.enableRustExpect,
    ),
    'pseudo_manual/basic_optional_test.dart': _generateBasicRelated(
      package,
      postfix: '_optional',
      imports: """
      import 'package:${package.dartPackageName}/src/rust/api/pseudo_manual/basic.dart';
      """,
      values: (ty) =>
          ["null", ...ty.interestRawValues.map((x) => x.textAndGuard)],
      valueType: (ty) => '${ty.dartTypeName}?',
    ),
    'pseudo_manual/basic_list_test.dart': _generateBasicRelated(
      package,
      postfix: '_list',
      values: (ty) => [
        ty.listWrapper(ty, null),
        ...ty.interestRawValues
            .map((x) => x.guard + ty.listWrapper(ty, x.text)),
      ],
      imports: """
      import 'package:${package.dartPackageName}/src/rust/api/pseudo_manual/basic.dart';
      """,
      enable: (ty) => ty.enableList,
      valueType: (ty) => ty.listName,
    ),
    'pseudo_manual/basic_map_test.dart': _generateBasicRelated(
      package,
      postfix: '_map',
      imports: """
      import 'package:${package.dartPackageName}/src/rust/api/pseudo_manual/basic.dart';
      """,
      values: (ty) => [
        '{}',
        ...ty.interestRawValues.map((x) => '${x.guard}{42: ${x.text}}')
      ],
      valueType: (ty) => 'Map<int, ${ty.dartTypeName}>',
    ),
    if (package == Package.pureDart)
      '../../benchmark/src/generated.dart': generateBenchmark(),
  };
}

String _generateBasicRelated(
  Package package, {
  required String postfix,
  required List<String> Function(BasicTypeInfo) values,
  required String? Function(BasicTypeInfo) valueType,
  bool Function(BasicTypeInfo)? withExpect,
  String imports = '',
  bool Function(BasicTypeInfo)? enable,
}) {
  final builder = DartFileBuilder(package, importName: 'basic$postfix');
  builder.imports += '''
  $imports
  ''';
  for (final ty in kBasicTypes) {
    if (enable?.call(ty) ?? true) {
      builder.addTestsIdentityFunctionCall(
        'exampleBasic${ReCase(postfix).pascalCase}Type${ReCase(ty.name).pascalCase}TwinNormal',
        values(ty),
        valueType: valueType(ty),
        withExpect: withExpect?.call(ty) ?? false,
      );
    }
  }
  return builder.toString();
}
