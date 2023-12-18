import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/rust/builder.dart';

Map<String, String> generateRustDirectSources() {
  return {
    'pseudo_manual/primitive.rs': _generatePrimitive(),
    'pseudo_manual/optional_primitive.rs': _generateOptionalPrimitive(),
    'pseudo_manual/primitive_list.rs': _generatePrimitiveList(),
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
    builder.addIdentityFunction(
        'Vec<${ty.name}>', 'example_primitive_list_type_${ty.name}');
  }
  return builder.toString();
}

String _generateOptionalPrimitive() {
  final builder = RustFileBuilder();
  for (final ty in kPrimitiveTypes) {
    builder.addIdentityFunction(
        'Option<${ty.name}>', 'example_optional_primitive_type_${ty.name}');
  }
  return builder.toString();
}

class PrimitiveTypeInfo {
  final String name;
  final String dartTypeName;
  final String primitiveListName;
  final List<String> interestRawValues;
  final String Function(PrimitiveTypeInfo, String) primitiveWrapper;
  final String Function(PrimitiveTypeInfo, String) primitiveListWrapper;

  const PrimitiveTypeInfo({
    required this.name,
    required this.dartTypeName,
    required this.primitiveListName,
    required this.interestRawValues,
    this.primitiveWrapper = _defaultPrimitiveWrapper,
    this.primitiveListWrapper = _defaultPrimitiveListWrapper,
  });

  static String _defaultPrimitiveWrapper(
          PrimitiveTypeInfo info, String value) =>
      value;

  static String _defaultPrimitiveListWrapper(
          PrimitiveTypeInfo info, String value) =>
      '${info.primitiveListName}.fromList([$value])';
}

final kPrimitiveTypes = [
  const PrimitiveTypeInfo(
    name: 'i8',
    dartTypeName: 'int',
    primitiveListName: 'Int8List',
    interestRawValues: ['0', '-128', '127'],
  ),
  const PrimitiveTypeInfo(
    name: 'i16',
    dartTypeName: 'int',
    primitiveListName: 'Int16List',
    interestRawValues: ['0', '-32768', '32767'],
  ),
  const PrimitiveTypeInfo(
    name: 'i32',
    dartTypeName: 'int',
    primitiveListName: 'Int32List',
    interestRawValues: ['0', '-2147483648', '2147483647'],
  ),
  const PrimitiveTypeInfo(
    name: 'i64',
    // dartTypeName: 'BigInt',
    dartTypeName: 'int',
    primitiveListName: 'Int64List',
    interestRawValues: [
      '0',
      '-9007199254740992',
      '9007199254740992',
      // TODO handle >53bit values, in dart web compiler it will error
      // '-9223372036854775808',
      // '9223372036854775807',
    ],
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  const PrimitiveTypeInfo(
    name: 'u8',
    dartTypeName: 'int',
    primitiveListName: 'Uint8List',
    interestRawValues: ['0', '255'],
  ),
  const PrimitiveTypeInfo(
    name: 'u16',
    dartTypeName: 'int',
    primitiveListName: 'Uint16List',
    interestRawValues: ['0', '65535'],
  ),
  const PrimitiveTypeInfo(
    name: 'u32',
    dartTypeName: 'int',
    primitiveListName: 'Uint32List',
    interestRawValues: ['0', '4294967295'],
  ),
  const PrimitiveTypeInfo(
    name: 'u64',
    // dartTypeName: 'BigInt',
    dartTypeName: 'int',
    primitiveListName: 'Uint64List',
    // '18446744073709551615', // not support numbers bigger than max i64 yet (but implementable)
    interestRawValues: [
      '0',
      '9007199254740992',
      // TODO handle >53bit values, in dart web compiler it will error
      // '9223372036854775807',
    ],
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  const PrimitiveTypeInfo(
    name: 'f32',
    dartTypeName: 'double',
    primitiveListName: 'Float32List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  const PrimitiveTypeInfo(
    name: 'f64',
    dartTypeName: 'double',
    primitiveListName: 'Float64List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  PrimitiveTypeInfo(
    name: 'bool',
    dartTypeName: 'bool',
    primitiveListName: 'List<bool>',
    interestRawValues: ['false', 'true'],
    primitiveListWrapper: (info, x) => '<bool>[$x]',
  ),
];
