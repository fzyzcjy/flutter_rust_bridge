@TestOn('vm')
library;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import '../../dart_minimal/lib/src/rust/api/minimal.dart' as minimal_api;
import '../../dart_minimal/lib/src/rust/frb_generated.dart' as minimal;

import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_fn.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

void main() {
  test('two generated FRB packages share one native library', () async {
    final library = await loadExternalLibrary(
      RustLib.kDefaultExternalLibraryLoaderConfig,
    );
    await RustLib.init(externalLibrary: library);
    await minimal.RustLib.init(externalLibrary: library);
    addTearDown(() {
      minimal.RustLib.dispose();
      RustLib.dispose();
    });

    // Both initializers check their own (different) content hashes.
    expect(
      RustLib.instance.rustContentHash,
      isNot(minimal.RustLib.instance.rustContentHash),
    );
    expect(await simpleAdderTwinNormal(a: 2, b: 3), 5);
    expect(await minimal_api.minimalAdder(a: 20, b: 30), 50);
    expect(minimal_api.minimalSyncAdder(a: 40, b: 50), 90);
    expect(await minimal_api.minimalCallback(callback: () => 123), 123);
    expect(await simpleAdderTwinNormal(a: 7, b: 8), 15);
    var callbacks = 0;
    await rustCallDartSimpleTwinNormal(callback: () => callbacks++);
    expect(callbacks, 1);
  });
}
