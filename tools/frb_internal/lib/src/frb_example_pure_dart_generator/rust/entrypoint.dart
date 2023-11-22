import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';
import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';

Future<void> generateRust({required Uri rustRoot}) async {
  final textOfPathMap = {
    'src/api/primitive.rs': _generateSrcApiPrimitive(),
    'src/api/optional_primitive.rs': _generateSrcApiOptionalPrimitive(),
  };

  writeCodeFiles(rustRoot, textOfPathMap);
  await executeRustFormat(workingDirectory: rustRoot.toFilePath());
}

String _generateSrcApiPrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    final name = ty.name;
    builder.body += 'pub fn example_primitive_type_$name(arg: $name) -> $name { arg }\n';
  }
  return builder.toString();
}

String _generateSrcApiOptionalPrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    final name = ty.name;
    builder.body += 'pub fn example_optional_primitive_type_$name(arg: Option<$name>) -> Option<$name> { arg }\n';
  }
  return builder.toString();
}

class PrimitiveTypeInfo {
  final String name;
  final List<String> interestValues;

  const PrimitiveTypeInfo(this.name, this.interestValues);
}

const kPrimitiveTypes = [
  PrimitiveTypeInfo('i8', ['0', '-128', '127']),
  PrimitiveTypeInfo('i16', ['0', '-32768', '32767']),
  PrimitiveTypeInfo('i32', ['0', '-2147483648', '2147483647']),
  PrimitiveTypeInfo(
      'i64', ['BigInt.parse("0")', 'BigInt.parse("-9223372036854775808")', 'BigInt.parse("9223372036854775807")']),
  PrimitiveTypeInfo('u8', ['0', '255']),
  PrimitiveTypeInfo('u16', ['0', '65535']),
  PrimitiveTypeInfo('u32', ['0', '4294967295']),
  PrimitiveTypeInfo('u64', [
    'BigInt.parse("0")',
    // 'BigInt.parse("18446744073709551615")', // not support numbers bigger than max i64 yet (but implementable)
    'BigInt.parse("9223372036854775807")',
  ]),
  PrimitiveTypeInfo('f32', ['0', '-42.5', '123456']),
  PrimitiveTypeInfo('f64', ['0', '-42.5', '123456']),
  PrimitiveTypeInfo('bool', ['false', 'true']),
];
