import 'package:frb_example_pure_dart/src/rust/api/primitive_misc.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

// NOTE majority of tests are in `pseudo_manual/*`
Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call primitiveTypes', () async {
    expect(
        await primitiveTypes(
            myI32: 123, myI64: BigInt.from(10000000000000), myF64: 12345678901234567890.123, myBool: true),
        42);
  });

  // TODO rm?
  // test('dart call primitiveTypesSync', () {
  //   expect(primitiveTypesSync(myI32: 123, myI64: 10000000000000, myF64: 12345678901234567890.123, myBool: true), 42);
  // });

  test('dart call primitiveU32', () async {
    expect(await primitiveU32(myU32: 0xff112233), 0xfe112233);
  });

  // TODO rm?
  // test('dart call primitiveU32Sync', () {
  //   expect(primitiveU32Sync(myU32: 0xff112233), 0xfe112233);
  // });

  // temporarily disable, re-enable later
  // test('dart call getUsize', () async {
  //   expect(await getUsize(u: 2), 2);
  // });
}
