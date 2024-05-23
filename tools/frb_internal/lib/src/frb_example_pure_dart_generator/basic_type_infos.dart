import 'package:recase/recase.dart';

class BasicTypeInfo {
  final String name;
  final String rustTypeName;
  final String dartTypeName;
  final String listName;
  final bool enableList;
  final List<RawValue> interestRawValues;
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

class RawValue {
  final String text;
  final bool nonWebOnly;

  const RawValue(this.text, {this.nonWebOnly = false});
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
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-128'),
      const RawValue('127'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'i16',
    dartTypeName: 'int',
    listName: 'Int16List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-32768'),
      const RawValue('32767'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'i32',
    dartTypeName: 'int',
    listName: 'Int32List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-2147483648'),
      const RawValue('2147483647'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'i64',
    dartTypeName: 'PlatformInt64',
    listName: 'Int64List',
    interestRawValues: [
      RawValue(_platformInt64('0')),
      RawValue(_platformInt64('-9007199254740992')),
      RawValue(_platformInt64('9007199254740992')),
      RawValue(_platformInt64('-9223372036854775808')),
      RawValue(_platformInt64('9223372036854775807')),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  BasicTypeInfo(
    rustTypeName: 'i128',
    dartTypeName: 'BigInt',
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('-9007199254740992')),
      RawValue(_bigInt('9007199254740992')),
      RawValue(_bigInt('-9223372036854775808')),
      RawValue(_bigInt('9223372036854775807')),
      RawValue(_bigInt('-170141183460469231731687303715884105728')),
      RawValue(_bigInt('170141183460469231731687303715884105727')),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u8',
    dartTypeName: 'int',
    listName: 'Uint8List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('255'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u16',
    dartTypeName: 'int',
    listName: 'Uint16List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('65535'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u32',
    dartTypeName: 'int',
    listName: 'Uint32List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('4294967295'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'u64',
    dartTypeName: 'BigInt',
    listName: 'Uint64List',
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('9007199254740992')),
      RawValue(_bigInt('9223372036854775807')),
      RawValue(_bigInt('18446744073709551615')),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  BasicTypeInfo(
    rustTypeName: 'u128',
    dartTypeName: 'BigInt',
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('9007199254740992')),
      RawValue(_bigInt('18446744073709551615')),
      RawValue(_bigInt('340282366920938463463374607431768211455')),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'isize',
    dartTypeName: 'PlatformInt64',
    enableList: false,
    interestRawValues: [
      RawValue(_platformInt64('0')),
      RawValue(_platformInt64('-2147483648')),
      RawValue(_platformInt64('2147483647')),
      RawValue(_platformInt64('-9007199254740992'), nonWebOnly: true),
      RawValue(_platformInt64('9007199254740992'), nonWebOnly: true),
      RawValue(_platformInt64('-9223372036854775808'), nonWebOnly: true),
      RawValue(_platformInt64('9223372036854775807'), nonWebOnly: true),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'usize',
    dartTypeName: 'BigInt',
    enableList: false,
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('4294967295')),
      RawValue(_bigInt('9007199254740992'), nonWebOnly: true),
      RawValue(_bigInt('9223372036854775807'), nonWebOnly: true),
      RawValue(_bigInt('18446744073709551615'), nonWebOnly: true),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'f32',
    dartTypeName: 'double',
    listName: 'Float32List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-42.5'),
      const RawValue('123456'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'f64',
    dartTypeName: 'double',
    listName: 'Float64List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-42.5'),
      const RawValue('123456'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'bool',
    dartTypeName: 'bool',
    listName: 'List<bool>',
    interestRawValues: [
      const RawValue('false'),
      const RawValue('true'),
    ],
    listWrapper: (info, x) => '<bool>[$x]',
  ),
  BasicTypeInfo(
    rustTypeName: 'String',
    dartTypeName: 'String',
    interestRawValues: [
      const RawValue('""'),
      const RawValue('"hello"'),
      const RawValue('"😂"'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    name: 'bytes',
    rustTypeName: 'Vec<u8>',
    dartTypeName: 'Uint8List',
    interestRawValues: [
      const RawValue('Uint8List.fromList([])'),
      const RawValue('Uint8List.fromList([255, 0])'),
      const RawValue('Uint8List.fromList([10, 20, 30, 40])'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicPrimitiveEnumTwinNormal',
    dartTypeName: 'BasicPrimitiveEnumTwinNormal',
    interestRawValues: [
      const RawValue('BasicPrimitiveEnumTwinNormal.apple'),
      const RawValue('BasicPrimitiveEnumTwinNormal.orange'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicGeneralEnumTwinNormal',
    dartTypeName: 'BasicGeneralEnumTwinNormal',
    interestRawValues: [
      const RawValue('BasicGeneralEnumTwinNormal.apple(field: "one")'),
      const RawValue('BasicGeneralEnumTwinNormal.orange()'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
  BasicTypeInfo(
    rustTypeName: 'BasicStructTwinNormal',
    dartTypeName: 'BasicStructTwinNormal',
    interestRawValues: [
      const RawValue('BasicStructTwinNormal(apple: null, orange: null)'),
      const RawValue('BasicStructTwinNormal(apple: "one", orange: 42)'),
    ],
    listWrapper: _defaultGeneralListWrapper,
  ),
];

String _bigInt(String raw) => 'BigInt.parse("$raw")';

String _platformInt64(String raw) => 'PlatformInt64.parse("$raw")';
