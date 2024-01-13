import 'package:recase/recase.dart';

class BasicTypeInfo {
  final String name;
  final String rustTypeName;
  final String dartTypeName;
  final String? primitiveListName;
  final bool enableList;
  final List<String> interestRawValues;
  final String Function(BasicTypeInfo, String) primitiveWrapper;
  final String Function(BasicTypeInfo, String) primitiveListWrapper;

  BasicTypeInfo({
    String? name,
    required this.rustTypeName,
    required this.dartTypeName,
    this.primitiveListName,
    this.enableList = true,
    required this.interestRawValues,
    this.primitiveWrapper = _defaultPrimitiveWrapper,
    this.primitiveListWrapper = _defaultPrimitiveListWrapper,
  }) : name = name ?? ReCase(rustTypeName).snakeCase;

  static String _defaultPrimitiveWrapper(BasicTypeInfo info, String value) =>
      value;

  static String _defaultPrimitiveListWrapper(
          BasicTypeInfo info, String value) =>
      '${info.primitiveListName}.fromList([$value])';
}

final kBasicTypes = [
  BasicTypeInfo(
    rustTypeName: 'i8',
    dartTypeName: 'int',
    primitiveListName: 'Int8List',
    interestRawValues: ['0', '-128', '127'],
  ),
  BasicTypeInfo(
    rustTypeName: 'i16',
    dartTypeName: 'int',
    primitiveListName: 'Int16List',
    interestRawValues: ['0', '-32768', '32767'],
  ),
  BasicTypeInfo(
    rustTypeName: 'i32',
    dartTypeName: 'int',
    primitiveListName: 'Int32List',
    interestRawValues: ['0', '-2147483648', '2147483647'],
  ),
  BasicTypeInfo(
    rustTypeName: 'i64',
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
  BasicTypeInfo(
    rustTypeName: 'u8',
    dartTypeName: 'int',
    primitiveListName: 'Uint8List',
    interestRawValues: ['0', '255'],
  ),
  BasicTypeInfo(
    rustTypeName: 'u16',
    dartTypeName: 'int',
    primitiveListName: 'Uint16List',
    interestRawValues: ['0', '65535'],
  ),
  BasicTypeInfo(
    rustTypeName: 'u32',
    dartTypeName: 'int',
    primitiveListName: 'Uint32List',
    interestRawValues: ['0', '4294967295'],
  ),
  BasicTypeInfo(
    rustTypeName: 'u64',
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
  BasicTypeInfo(
    rustTypeName: 'isize',
    dartTypeName: 'int',
    enableList: false,
    interestRawValues: [
      '0',
      '-2147483648',
      '2147483647',
      '-9007199254740992',
      '9007199254740992',
    ],
  ),
  BasicTypeInfo(
    rustTypeName: 'usize',
    dartTypeName: 'int',
    enableList: false,
    interestRawValues: ['0', '4294967295', '9007199254740992'],
  ),
  BasicTypeInfo(
    rustTypeName: 'f32',
    dartTypeName: 'double',
    primitiveListName: 'Float32List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  BasicTypeInfo(
    rustTypeName: 'f64',
    dartTypeName: 'double',
    primitiveListName: 'Float64List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  BasicTypeInfo(
    rustTypeName: 'bool',
    dartTypeName: 'bool',
    primitiveListName: 'List<bool>',
    interestRawValues: ['false', 'true'],
    primitiveListWrapper: (info, x) => '<bool>[$x]',
  ),
  BasicTypeInfo(
    rustTypeName: 'String',
    dartTypeName: 'String',
    interestRawValues: ['""', '"hello"', '"😂"'],
  ),
  BasicTypeInfo(
    name: 'bytes',
    rustTypeName: 'Vec<u8>',
    dartTypeName: 'Uint8List',
    // TODO await upstream allo-isolate to implement
    enableList: false,
    interestRawValues: [
      'Uint8List.fromList([])',
      'Uint8List.fromList([255, 0])',
      'Uint8List.fromList([10, 20, 30, 40])'
    ],
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicPrimitiveEnumTwinNormal',
    dartTypeName: 'BasicPrimitiveEnumTwinNormal',
    interestRawValues: [
      'BasicPrimitiveEnumTwinNormal.apple',
      'BasicPrimitiveEnumTwinNormal.orange',
    ],
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicGeneralEnumTwinNormal',
    dartTypeName: 'BasicGeneralEnumTwinNormal',
    interestRawValues: [
      'BasicGeneralEnumTwinNormal.apple(field: "one")',
      'BasicGeneralEnumTwinNormal.orange()',
    ],
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicStructTwinNormal',
    dartTypeName: 'BasicStructTwinNormal',
    interestRawValues: [
      'BasicStructTwinNormal(apple: null, orange: null)',
      'BasicStructTwinNormal(apple: "one", orange: 42)',
    ],
  ),
];
