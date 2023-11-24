import 'package:frb_example_pure_dart/src/rust/api/raw_string.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('test dart raw string in struct', () async {
    final output = await testRawStringItemStruct();
    expect(output, isA<RawStringItemStruct>());
    expect(output.type, "test");
  });

  // test('test dart raw string in struct with raw func', () async {
  //   final output = await testRawStringItemStructWithRawStringInFunc("not a type ;')");
  //   expect(output.type, "not a type ;')");
  // });

  test('test dart test more than just one raw string struct', () async {
    final output = await testMoreThanJustOneRawStringStruct();
    expect(output, isA<MoreThanJustOneRawStringStruct>());
    expect(output.regular, "regular");
    expect(output.type, "type");
    expect(output.async, true);
    expect(output.another, "another");
  });
}
