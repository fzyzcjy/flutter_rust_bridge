import 'package:frb_example_pure_dart/src/rust/api/tuple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test("dart call tuples", () async {
    expect(await testTupleTwinNormal(), ('John', 0));
    expect(await testTupleTwinNormal(value: ('Bob', 42)), ('Hello Bob', 43));
  });

  test("dart call optional f32 tuple returns record", () async {
    expect(await returnOptionalF32TupleTwinNormal(), (1.25, 2.5));
  });
}
