@TestOn('vm')
library;

import 'dart:ffi';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_fn.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../native_symbol_prefix/lib/src/rust/api.dart'
    as native_symbol_prefix_api;
import '../../native_symbol_prefix/lib/src/rust/frb_generated.dart'
    as native_symbol_prefix;

void main() {
  test('two generated FRB packages share one native library', () async {
    final library = await loadExternalLibrary(
      RustLib.kDefaultExternalLibraryLoaderConfig,
    );
    await RustLib.init(externalLibrary: library);
    await native_symbol_prefix.RustLib.init(externalLibrary: library);
    addTearDown(() {
      native_symbol_prefix.RustLib.dispose();
      RustLib.dispose();
    });

    // Both initializers validate their generated hash against their own export.
    int nativeHash(String symbol) => library.ffiDynamicLibrary
        .lookupFunction<Int32 Function(), int Function()>(symbol)();
    expect(
      nativeHash('frb_get_rust_content_hash'),
      isNot(nativeHash('example_2nd_pkg_frb_get_rust_content_hash')),
    );
    expect(await simpleAdderTwinNormal(a: 2, b: 3), 5);
    expect(await native_symbol_prefix_api.add(a: 20, b: 30), 50);
    expect(native_symbol_prefix_api.addSync(a: 40, b: 50), 90);
    expect(await native_symbol_prefix_api.callDart(callback: () => 123), 123);
    expect(await simpleAdderTwinNormal(a: 7, b: 8), 15);
    var callbacks = 0;
    await rustCallDartSimpleTwinNormal(callback: () => callbacks++);
    expect(callbacks, 1);
  });
}
