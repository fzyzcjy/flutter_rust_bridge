class BasicTypeInfo {
  final String rustTypeName;
  final String dartTypeName;
  final String? primitiveListName;
  final List<String> interestRawValues;
  final String Function(BasicTypeInfo, String) primitiveWrapper;
  final String Function(BasicTypeInfo, String) primitiveListWrapper;

  const BasicTypeInfo({
    required this.rustTypeName,
    required this.dartTypeName,
    this.primitiveListName,
    required this.interestRawValues,
    this.primitiveWrapper = _defaultPrimitiveWrapper,
    this.primitiveListWrapper = _defaultPrimitiveListWrapper,
  });

  static String _defaultPrimitiveWrapper(BasicTypeInfo info, String value) =>
      value;

  static String _defaultPrimitiveListWrapper(
          BasicTypeInfo info, String value) =>
      '${info.primitiveListName}.fromList([$value])';
}

final kBasicTypes = [
  const BasicTypeInfo(
    rustTypeName: 'i8',
    dartTypeName: 'int',
    primitiveListName: 'Int8List',
    interestRawValues: ['0', '-128', '127'],
  ),
  const BasicTypeInfo(
    rustTypeName: 'i16',
    dartTypeName: 'int',
    primitiveListName: 'Int16List',
    interestRawValues: ['0', '-32768', '32767'],
  ),
  const BasicTypeInfo(
    rustTypeName: 'i32',
    dartTypeName: 'int',
    primitiveListName: 'Int32List',
    interestRawValues: ['0', '-2147483648', '2147483647'],
  ),
  const BasicTypeInfo(
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
  const BasicTypeInfo(
    rustTypeName: 'u8',
    dartTypeName: 'int',
    primitiveListName: 'Uint8List',
    interestRawValues: ['0', '255'],
  ),
  const BasicTypeInfo(
    rustTypeName: 'u16',
    dartTypeName: 'int',
    primitiveListName: 'Uint16List',
    interestRawValues: ['0', '65535'],
  ),
  const BasicTypeInfo(
    rustTypeName: 'u32',
    dartTypeName: 'int',
    primitiveListName: 'Uint32List',
    interestRawValues: ['0', '4294967295'],
  ),
  const BasicTypeInfo(
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
  const BasicTypeInfo(
    rustTypeName: 'isize',
    dartTypeName: 'int',
    primitiveListName: 'Int64List',
    interestRawValues: [
      '0',
      '-2147483648',
      '2147483647',
      '-9007199254740992',
      '9007199254740992',
    ],
  ),
  const BasicTypeInfo(
    rustTypeName: 'usize',
    dartTypeName: 'int',
    primitiveListName: 'Uint64List',
    interestRawValues: ['0', '4294967295', '9007199254740992'],
  ),
  const BasicTypeInfo(
    rustTypeName: 'f32',
    dartTypeName: 'double',
    primitiveListName: 'Float32List',
    interestRawValues: ['0', '-42.5', '123456'],
  ),
  const BasicTypeInfo(
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
  const BasicTypeInfo(
    rustTypeName: 'String',
    dartTypeName: 'String',
    interestRawValues: ['""', '"hello"', '"😂"'],
  ),
  const BasicTypeInfo(
    rustTypeName: 'Bytes',
    dartTypeName: 'Uint8List',
    interestRawValues: [
      'Uint8List.fromList([])',
      'Uint8List.fromList([255, 0])',
      'Uint8List.fromList([10, 20, 30, 40])'
    ],
  ),
  const BasicTypeInfo(
    rustTypeName: 'BasicPrimitiveEnum',
    dartTypeName: 'BasicPrimitiveEnum',
    interestRawValues: [
      'BasicPrimitiveEnum.apple',
      'BasicPrimitiveEnum.orange',
    ],
  ),
  const BasicTypeInfo(
    rustTypeName: 'BasicGeneralEnum',
    dartTypeName: 'BasicGeneralEnum',
    interestRawValues: [
      'BasicGeneralEnum.apple(field: "one")',
      'BasicGeneralEnum.orange()',
    ],
  ),
  const BasicTypeInfo(
    rustTypeName: 'BasicStruct',
    dartTypeName: 'BasicStruct',
    interestRawValues: [
      'BasicStruct(apple: null, orange: null)',
      'BasicStruct(apple: "one", orange: 42)',
    ],
  ),
];
