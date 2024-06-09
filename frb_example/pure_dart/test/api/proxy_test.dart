// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/proxy.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('simple proxy', () async {
    final node = await MyNodeTwinNormal.createTwinNormal();
    final paramOne = await node.paramOneTwinNormal();
    final paramTwo = await node.paramTwoTwinNormal();
    expect(await paramOne.myMethodTwinNormal(), 'aa');
    expect(await paramTwo.myMethodTwinNormal(), 'bb');
  });
}
