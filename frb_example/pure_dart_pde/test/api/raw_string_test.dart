// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/raw_string.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('test dart raw string in struct', () async {
    final output = await testRawStringItemStructTwinNormal();
    expect(output, isA<RawStringItemStructTwinNormal>());
    expect(output.type, "test");
  });

  // test('test dart raw string in struct with raw func', () async {
  //   final output = await testRawStringItemStructWithRawStringInFunc("not a type ;')");
  //   expect(output.type, "not a type ;')");
  // });

  test('test dart test more than just one raw string struct', () async {
    final output = await testMoreThanJustOneRawStringStructTwinNormal();
    expect(output, isA<MoreThanJustOneRawStringStructTwinNormal>());
    expect(output.regular, "regular");
    expect(output.type, "type");
    expect(output.async_, true);
    expect(output.another, "another");
  });
}
