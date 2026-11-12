@TestOn('vm')
library;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_fn.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../native_symbol_prefix/lib/src/rust/api.dart' as fixture_api;
import '../../native_symbol_prefix/lib/src/rust/frb_generated.dart' as fixture;

void main() {
  test('two generated FRB packages share one native library', () async {
    final library = await loadExternalLibrary(
      RustLib.kDefaultExternalLibraryLoaderConfig,
    );
    await RustLib.init(externalLibrary: library);
    await fixture.RustLib.init(externalLibrary: library);
    addTearDown(() {
      fixture.RustLib.dispose();
      RustLib.dispose();
    });

    // Both initializers check their own (different) content hashes.
    expect(
      RustLib.instance.rustContentHash,
      isNot(fixture.RustLib.instance.rustContentHash),
    );
    expect(await simpleAdderTwinNormal(a: 2, b: 3), 5);
    expect(await fixture_api.add(a: 20, b: 30), 50);
    expect(fixture_api.addSync(a: 40, b: 50), 90);
    expect(await fixture_api.callDart(callback: () => 123), 123);
    expect(await simpleAdderTwinNormal(a: 7, b: 8), 15);
    var callbacks = 0;
    await rustCallDartSimpleTwinNormal(callback: () => callbacks++);
    expect(callbacks, 1);
  });
}
