// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `raw_string_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/raw_string_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('test dart raw string in struct', () async {
    final output = await testRawStringItemStructTwinSync();
    expect(output, isA<RawStringItemStructTwinSync>());
    expect(output.type, "test");
  });

  // test('test dart raw string in struct with raw func', () async {
  //   final output = await testRawStringItemStructWithRawStringInFunc("not a type ;')");
  //   expect(output.type, "not a type ;')");
  // });

  test('test dart test more than just one raw string struct', () async {
    final output = await testMoreThanJustOneRawStringStructTwinSync();
    expect(output, isA<MoreThanJustOneRawStringStructTwinSync>());
    expect(output.regular, "regular");
    expect(output.type, "type");
    expect(output.async_, true);
    expect(output.another, "another");
  });
}
