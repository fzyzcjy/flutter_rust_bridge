import 'package:frb_example_pure_dart/src/rust/api/newtype_pattern.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handleNewtype', () async {
    final newtypeResp = await handleNewtype(arg: NewTypeInt(field0: BigInt.from(42)));
    expect(newtypeResp.field0.toInt(), 84);
  });

  // TODO rm?
  // test('dart call handleNewtypeSync', () {
  //   final newtypeResp = handleNewtypeSync(arg: NewTypeInt(field0: 42));
  //   expect(newtypeResp.field0, 84);
  // });
}
