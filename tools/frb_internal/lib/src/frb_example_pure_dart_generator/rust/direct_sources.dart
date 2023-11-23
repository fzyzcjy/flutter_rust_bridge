import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/primitive.rs': _generatePrimitive(),
    'pseudo_manual/optional_primitive.rs': _generateOptionalPrimitive(),
    'pseudo_manual/primitive_list.rs': _generatePrimitiveList(),
//  TODO
//     'pseudo_manual/structure.rs': _generateStructure(),
  };
}

String _generatePrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    builder.addIdentityFunction(ty.name, 'example_primitive_type_${ty.name}');
  }
  return builder.toString();
}

String _generatePrimitiveList() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    builder.addIdentityFunction('Vec<${ty.name}>', 'example_primitive_list_type_${ty.name}');
  }
  return builder.toString();
}

String _generateOptionalPrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    builder.addIdentityFunction('Option<${ty.name}>', 'example_optional_primitive_type_${ty.name}');
  }
  return builder.toString();
}

//  TODO
// // TODO corresponding tests in dart
// String _generateStructure() {
//   final builder = RustFileBuilder();
//
//   builder.body += '''
// pub struct StructSimpleTwinNormal {
//     pub field: i32,
// }
//
// #[frb(dart_metadata = ("freezed"))]
// pub struct StructFreezedTwinNormal {
//     pub field: i32,
// }
//   ''';
//
//   for (final (ty, partialName) in [
//     ('StructSimpleTwinNormal', 'func_struct_simple'),
//     ('StructFreezedTwinNormal', 'func_struct_freezed'),
//   ]) {
//     builder.addIdentityFunction(ty, partialName);
//     builder.addIdentityFunction('Box<$ty>', '${partialName}_boxed');
//   }
//
//   return builder.toString();
// }

class PrimitiveTypeInfo {
  final String name;
  final String primitiveListName;
  final List<String> interestRawValues;
  final String Function(PrimitiveTypeInfo, String) primitiveWrapper;
  final String Function(PrimitiveTypeInfo, String) primitiveListWrapper;

  const PrimitiveTypeInfo({
    required this.name,
    required this.primitiveListName,
    required this.interestRawValues,
    this.primitiveWrapper = _defaultPrimitiveWrapper,
    this.primitiveListWrapper = _defaultPrimitiveListWrapper,
  });

  static String _defaultPrimitiveWrapper(PrimitiveTypeInfo info, String value) => value;

  static String _defaultPrimitiveListWrapper(PrimitiveTypeInfo info, String value) =>
      '${info.primitiveListName}.fromList([$value])';
}

final kPrimitiveTypes = [
  PrimitiveTypeInfo(
    name: 'i8',
    primitiveListName: 'Int8List',
    interestRawValues: ['0', '-128', '127'],
  ),
  PrimitiveTypeInfo(
    name: 'i16',
    primitiveListName: 'Int16List',
    interestRawValues: ['0', '-32768', '32767'],
  ),
  PrimitiveTypeInfo(
    name: 'i32',
    primitiveListName: 'Int32List',
    interestRawValues: ['0', '-2147483648', '2147483647'],
  ),
  PrimitiveTypeInfo(
    name: 'i64',
    primitiveListName: 'Int64List',
    interestRawValues: ['0', '-9223372036854775808', '9223372036854775807'],
    primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  PrimitiveTypeInfo(
    name: 'u8',
    primitiveListName: 'Uint8List',
    interestRawValues: ['0', '255'],
  ),
  PrimitiveTypeInfo(
    name: 'u16',
    primitiveListName: 'Uint16List',
    interestRawValues: ['0', '65535'],
  ),
  PrimitiveTypeInfo(
    name: 'u32',
    primitiveListName: 'Uint32List',
    interestRawValues: ['0', '4294967295'],
  ),
  PrimitiveTypeInfo(
    name: 'u64',
    primitiveListName: 'Uint64List',
    // '18446744073709551615', // not support numbers bigger than max i64 yet (but implementable)
    interestRawValues: ['0', '9223372036854775807'],
    primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  PrimitiveTypeInfo(
    name: 'f32',
    primitiveListName: 'Float32List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  PrimitiveTypeInfo(
    name: 'f64',
    primitiveListName: 'Float64List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  PrimitiveTypeInfo(
    name: 'bool',
    primitiveListName: 'List<bool>',
    interestRawValues: ['false', 'true'],
    primitiveListWrapper: (info, x) => '<bool>[$x]',
  ),
];
