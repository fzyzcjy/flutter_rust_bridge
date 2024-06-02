import 'package:recase/recase.dart';

class BasicTypeInfo {
  final String name;
  final String rustTypeName;
  final String dartTypeName;
  final String listName;
  final bool enableList;
  final bool enableRustExpect;
  final List<RawValue> interestRawValues;
  final String Function(BasicTypeInfo, String?) listWrapper;

  BasicTypeInfo({
    String? name,
    required this.rustTypeName,
    required this.dartTypeName,
    String? listName,
    this.enableList = true,
    this.enableRustExpect = false,
    required this.interestRawValues,
    required this.listWrapper,
  })  : name = name ?? ReCase(rustTypeName).snakeCase,
        listName = listName ?? 'List<$dartTypeName>';
}

class RawValue {
  final String text;
  final bool nonWebOnly;

  const RawValue(this.text, {this.nonWebOnly = false});

  String get guard => nonWebOnly ? "if (!kIsWeb) " : "";

  String get textAndGuard => '$guard$text';
}

String _defaultGeneralListWrapper(BasicTypeInfo info, String? value) =>
    value == null ? '[]' : '[$value]';

String _defaultPrimitiveListWrapper(BasicTypeInfo info, String? value) =>
    value == null ? '${info.listName}(0)' : '${info.listName}(1)..[0] = $value';

final kBasicTypes = [
  BasicTypeInfo(
    rustTypeName: 'i8',
    dartTypeName: 'int',
    listName: 'Int8List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-128'),
      const RawValue('127'),
      const RawValue('79'),
      const RawValue('-79'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'i16',
    dartTypeName: 'int',
    listName: 'Int16List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-32768'),
      const RawValue('32767'),
      const RawValue('12345'),
      const RawValue('-12345'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'i32',
    dartTypeName: 'int',
    listName: 'Int32List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('-2147483648'),
      const RawValue('2147483647'),
      const RawValue('1234567890'),
      const RawValue('-1234567890'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
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
      RawValue(_platformInt64('1234567890123456789')),
      RawValue(_platformInt64('-1234567890123456789')),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  BasicTypeInfo(
    rustTypeName: 'i128',
    dartTypeName: 'BigInt',
    enableList: false,
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
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'u8',
    dartTypeName: 'int',
    listName: 'Uint8List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('255'),
      const RawValue('123'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'u16',
    dartTypeName: 'int',
    listName: 'Uint16List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('65535'),
      const RawValue('12345'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'u32',
    dartTypeName: 'int',
    listName: 'Uint32List',
    interestRawValues: [
      const RawValue('0'),
      const RawValue('4294967295'),
      const RawValue('2468013579'),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'u64',
    dartTypeName: 'BigInt',
    listName: 'Uint64List',
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('9007199254740992')),
      RawValue(_bigInt('9223372036854775807')),
      RawValue(_bigInt('9223372036854775808')),
      RawValue(_bigInt('18446744073709551615')),
      RawValue(_bigInt('12345678912345678913')),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
    // primitiveWrapper: (_, x) => 'BigInt.parse("$x")',
  ),
  BasicTypeInfo(
    rustTypeName: 'u128',
    dartTypeName: 'BigInt',
    enableList: false,
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('9007199254740992')),
      RawValue(_bigInt('9223372036854775807')),
      RawValue(_bigInt('9223372036854775808')),
      RawValue(_bigInt('18446744073709551615')),
      RawValue(_bigInt('340282366920938463463374607431768211455')),
    ],
    listWrapper: _defaultGeneralListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'isize',
    dartTypeName: 'PlatformInt64',
    enableList: false,
    interestRawValues: [
      RawValue(_platformInt64('0')),
      RawValue(_platformInt64('-2147483648')),
      RawValue(_platformInt64('2147483647')),
      RawValue(_platformInt64('-1234234567')),
      RawValue(_platformInt64('1234234567')),
      // NOTE web is currently 32bit, thus `isize`/`usize` is 32 bit
      RawValue(_platformInt64('-9007199254740992'), nonWebOnly: true),
      RawValue(_platformInt64('9007199254740992'), nonWebOnly: true),
      RawValue(_platformInt64('-9223372036854775808'), nonWebOnly: true),
      RawValue(_platformInt64('9223372036854775807'), nonWebOnly: true),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
  ),
  BasicTypeInfo(
    rustTypeName: 'usize',
    dartTypeName: 'BigInt',
    enableList: false,
    interestRawValues: [
      RawValue(_bigInt('0')),
      RawValue(_bigInt('4294967295')),
      RawValue(_bigInt('1234234567')),
      RawValue(_bigInt('9007199254740992'), nonWebOnly: true),
      RawValue(_bigInt('9223372036854775807'), nonWebOnly: true),
      RawValue(_bigInt('18446744073709551615'), nonWebOnly: true),
      RawValue(_bigInt('12345678912345678913'), nonWebOnly: true),
    ],
    listWrapper: _defaultPrimitiveListWrapper,
    enableRustExpect: true,
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
    listWrapper: (info, x) => x == null ? '<bool>[]' : '<bool>[$x]',
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
