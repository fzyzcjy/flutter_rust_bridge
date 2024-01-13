import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/basic.rs': _generateBasicRelated((x) => x),
    'pseudo_manual/optional_basic.rs':
        _generateBasicRelated((x) => 'Option<$x>'),
    'pseudo_manual/basic_list.rs': _generateBasicRelated((x) => 'Vec<$x>'),
    'pseudo_manual/basic_map.rs':
        _generateBasicRelated((x) => 'HashMap<i32, $x>'),
    'pseudo_manual/benchmark_api.rs': generateBenchmark(),
  };
}

String _generateBasicRelated(String Function(String) rustTypeNameWrapper) {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(
        rustTypeNameWrapper(ty.rustTypeName), 'example_basic_type_${ty.name}');
  }
  return builder.toString();
}
