import 'package:frb_example_pure_dart/src/rust/api/dart_code.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('StructWithDartCodeTwinNormal', () async {
    final one = StructWithDartCodeTwinNormal(a: 100);
    final two = StructWithDartCodeTwinNormal(a: 100);
    expect(one.hashCode, two.hashCode);
    expect(one == two, true);
    expect(one.dartExtraMethod(), 200);
    expect(await one.normalMethod(), 200);
  });
}
