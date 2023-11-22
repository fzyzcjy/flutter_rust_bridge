import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';

Future<void> generateRust({required Uri rustRoot}) async {
  final textOfPathMap = {
    'src/api/primitive.rs': _generateSrcApiPrimitive(),
  };

  writeCodeFiles(rustRoot, textOfPathMap);
  await executeRustFormat(workingDirectory: rustRoot.toFilePath());
}

String _generateSrcApiPrimitive() {
  var ans = '';
  for (final ty in [
    'i8',
    'i16',
    'i32',
    'i64',
    'u8',
    'u16',
    'u32',
    'u64',
    'f32',
    'f64',
    'bool',
  ]) {
    ans += 'pub fn example_primitive_type_$ty(arg: $ty) -> $ty { $ty }\n';
  }
  return ans;
}
