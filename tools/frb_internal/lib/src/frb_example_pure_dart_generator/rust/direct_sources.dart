import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/basic.rs': _generateBasicRelated((x) => x, '',
        withArgExpect: (ty) => ty.enableRustExpect),
    'pseudo_manual/basic_optional.rs':
        _generateBasicRelated((x) => 'Option<$x>', '_optional'),
    'pseudo_manual/basic_list.rs': _generateBasicRelated(
      (x) => 'Vec<$x>',
      '_list',
      enable: (ty) => ty.enableList,
    ),
    'pseudo_manual/basic_map.rs': _generateBasicRelated(
        (x) => 'HashMap<i32, $x>', '_map',
        extraBody: 'use std::collections::HashMap;\n'),
    'pseudo_manual/benchmark_api.rs': generateBenchmark(),
  };
}

String _generateBasicRelated(
  String Function(String) rustTypeNameWrapper,
  String postfix, {
  String extraBody = '',
  bool Function(BasicTypeInfo)? enable,
  bool Function(BasicTypeInfo)? withArgExpect,
}) {
  final builder = RustFileBuilder();
  builder.body += extraBody;

  if (postfix.isEmpty) {
    builder.body += '''
pub enum BasicPrimitiveEnumTwinNormal {
    Apple,
    Orange,
}

pub enum BasicGeneralEnumTwinNormal {
    Apple { field: String },
    Orange,
}

pub struct BasicStructTwinNormal {
    pub apple: Option<String>,
    pub orange: Option<i32>,
}
''';
  } else {
    builder.body += '''
pub use super::basic::*;
    ''';
  }
  for (final ty in kBasicTypes) {
    if (enable?.call(ty) ?? true) {
      final partialName = 'example_basic${postfix}_type_${ty.name}';
      if (withArgExpect?.call(ty) ?? false) {
        builder.body +=
            '''pub fn ${partialName}_twin_normal(arg: ${ty.rustTypeName}, expect: String) -> ${ty.rustTypeName} {
          assert_eq!(arg, expect.parse::<${ty.rustTypeName}>().unwrap());
          arg
        }\n\n''';
      } else {
        builder.addIdentityFunction(
            rustTypeNameWrapper(ty.rustTypeName), partialName);
      }
    }
  }
  return builder.toString();
}
