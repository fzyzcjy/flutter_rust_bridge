// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/async_spawn.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call simpleUseAsyncSpawn', () async {
    expect(await simpleUseAsyncSpawn(arg: 'a'), 'aa');
  });

  test('dart call simpleUseAsyncSpawnBlocking', () async {
    expect(await simpleUseAsyncSpawnBlocking(arg: 'a'), 'aa');
  });

  test('dart call simpleUseAsyncSpawnLocal', () async {
    expect(await simpleUseAsyncSpawnLocal(arg: 'a').first, 'aa');
  });
}
