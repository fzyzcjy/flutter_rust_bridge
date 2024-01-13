import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/basic.rs': _generateBasicRelated((x) => x, ''),
    'pseudo_manual/basic_optional.rs':
        _generateBasicRelated((x) => 'Option<$x>', '_optional'),
    'pseudo_manual/basic_list.rs':
        _generateBasicRelated((x) => 'Vec<$x>', '_list'),
    'pseudo_manual/basic_map.rs':
        _generateBasicRelated((x) => 'HashMap<i32, $x>', '_map'),
    'pseudo_manual/benchmark_api.rs': generateBenchmark(),
  };
}

String _generateBasicRelated(
    String Function(String) rustTypeNameWrapper, String postfix) {
  final builder = RustFileBuilder();
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
    apple: String,
    orange: i32,
}
''';
  } else {
    builder.body += '''
pub use super::basic::*;
    ''';
  }
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(rustTypeNameWrapper(ty.rustTypeName),
        'example_basic${postfix}_type_${ty.name}');
  }
  return builder.toString();
}
