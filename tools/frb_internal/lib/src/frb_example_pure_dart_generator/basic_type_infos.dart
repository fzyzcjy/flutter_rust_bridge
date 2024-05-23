import 'package:recase/recase.dart';

class BasicTypeInfo {
  final String name;
  final String rustTypeName;
  final String dartTypeName;
  final String listName;
  final bool enableList;
  final List<String> interestRawValues;
  final String Function(BasicTypeInfo, String) listWrapper;

  BasicTypeInfo({
    String? name,
    required this.rustTypeName,
    required this.dartTypeName,
    String? listName,
    this.enableList = true,
    required this.interestRawValues,
    required this.listWrapper,
  })  : name = name ?? ReCase(rustTypeName).snakeCase,
        listName = listName ?? 'List<$dartTypeName>';
}

String _defaultGeneralListWrapper(BasicTypeInfo info, String value) =>
    '[$value]';

String _defaultPrimitiveListWrapper(BasicTypeInfo info, String value) =>
    '${info.listName}.fromList([$value])';

final kBasicTypes = [
  BasicTypeInfo(
    rustTypeName: 'i8',
    dartTypeName: 'int',
    listName: 'Int8List',
    interestRawValues: ['0', '-128', '127'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'i16',
    dartTypeName: 'int',
    listName: 'Int16List',
    interestRawValues: ['0', '-32768', '32767'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'i32',
    dartTypeName: 'int',
    listName: 'Int32List',
    interestRawValues: ['0', '-2147483648', '2147483647'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'i64',
    // dartTypeName: 'BigInt',
    dartTypeName: 'int',
    listName: 'Int64List',
    interestRawValues: [
      '0',
      '-9007199254740992',
      '9007199254740992',
      '-9223372036854775808',
      '9223372036854775807',
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  BasicTypeInfo(
    rustTypeName: 'i128',
    dartTypeName: 'BigInt',
    interestRawValues: [
      _bigInt('0'),
      _bigInt('-9007199254740992'),
      _bigInt('9007199254740992'),
      _bigInt('-9223372036854775808'),
      _bigInt('9223372036854775807'),
      _bigInt('-170141183460469231731687303715884105728'),
      _bigInt('170141183460469231731687303715884105727'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u8',
    dartTypeName: 'int',
    listName: 'Uint8List',
    interestRawValues: ['0', '255'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u16',
    dartTypeName: 'int',
    listName: 'Uint16List',
    interestRawValues: ['0', '65535'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u32',
    dartTypeName: 'int',
    listName: 'Uint32List',
    interestRawValues: ['0', '4294967295'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u64',
    dartTypeName: 'BigInt',
    listName: 'Uint64List',
    interestRawValues: [
      _bigInt('0'),
      _bigInt('9007199254740992'),
      _bigInt('9223372036854775807'),
      _bigInt('18446744073709551615'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  BasicTypeInfo(
    rustTypeName: 'u128',
    dartTypeName: 'BigInt',
    interestRawValues: [
      _bigInt('0'),
      _bigInt('9007199254740992'),
      _bigInt('18446744073709551615'),
      _bigInt('340282366920938463463374607431768211455'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'isize',
    dartTypeName: 'int',
    enableList: false,
    interestRawValues: ['0', '-2147483648', '2147483647'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'usize',
    dartTypeName: 'BigInt',
    enableList: false,
    interestRawValues: [
      _bigInt('0'),
      _bigInt('4294967295'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'f32',
    dartTypeName: 'double',
    listName: 'Float32List',
    interestRawValues: ['0', '-42.5', '123456'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'f64',
    dartTypeName: 'double',
    listName: 'Float64List',
    interestRawValues: ['0', '-42.5', '123456'],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'bool',
    dartTypeName: 'bool',
    listName: 'List<bool>',
    interestRawValues: ['false', 'true'],
    listWrapper: (info, x) => '<bool>[$x]',
  ),
  BasicTypeInfo(
    rustTypeName: 'String',
    dartTypeName: 'String',
    interestRawValues: ['""', '"hello"', '"😂"'],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    name: 'bytes',
    rustTypeName: 'Vec<u8>',
    dartTypeName: 'Uint8List',
    interestRawValues: [
      'Uint8List.fromList([])',
      'Uint8List.fromList([255, 0])',
      'Uint8List.fromList([10, 20, 30, 40])'
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicPrimitiveEnumTwinNormal',
    dartTypeName: 'BasicPrimitiveEnumTwinNormal',
    interestRawValues: [
      'BasicPrimitiveEnumTwinNormal.apple',
      'BasicPrimitiveEnumTwinNormal.orange',
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicGeneralEnumTwinNormal',
    dartTypeName: 'BasicGeneralEnumTwinNormal',
    interestRawValues: [
      'BasicGeneralEnumTwinNormal.apple(field: "one")',
      'BasicGeneralEnumTwinNormal.orange()',
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicStructTwinNormal',
    dartTypeName: 'BasicStructTwinNormal',
    interestRawValues: [
      'BasicStructTwinNormal(apple: null, orange: null)',
      'BasicStructTwinNormal(apple: "one", orange: 42)',
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
];

String _bigInt(String raw) => 'BigInt.parse("$raw")';
