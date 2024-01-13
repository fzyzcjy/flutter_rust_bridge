import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/basic_type_infos.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/benchmark.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/basic.rs': _generateBasic(),
    'pseudo_manual/optional_basic.rs': _generateOptionalBasic(),
    'pseudo_manual/basic_list.rs': _generateBasicList(),
    'pseudo_manual/basic_map.rs': _generateBasicMap(),
    'pseudo_manual/benchmark_api.rs': generateBenchmark(),
  };
}

String _generateBasic() {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(ty.name, 'example_basic_type_${ty.name}');
  }
  return builder.toString();
}

String _generateBasicList() {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(
        'Vec<${ty.name}>', 'example_basic_list_type_${ty.name}');
  }
  return builder.toString();
}

String _generateBasicMap() {
  throw UnimplementedError();
}

String _generateOptionalBasic() {
  final builder = RustFileBuilder();
  for (final ty in kBasicTypes) {
    builder.addIdentityFunction(
        'Option<${ty.name}>', 'example_optional_basic_type_${ty.name}');
  }
  return builder.toString();
}
