import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_pure_dart/src/rust/api/newtype_pattern.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call handleNewtype', () async {
    final newtypeResp = await handleNewtypeTwinNormal(
        arg: NewTypeIntTwinNormal(field0: PlatformInt64Util.from(42)));
    expect(newtypeResp.field0.toInt(), 84);
  });
}
