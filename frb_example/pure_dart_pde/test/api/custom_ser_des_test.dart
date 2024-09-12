// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'dart:io';

import 'package:frb_example_pure_dart_pde/src/rust/api/custom_ser_des.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('custom serializer', () async {
    expect(
        await functionUsingTypeWithCustomSerializer(arg: 123456789), 123456789);
  });

  group('Ipv4Addr', skip: kIsWeb, () {
    final addr = InternetAddress.tryParse('192.168.0.1')!;
    test('funcUsingIpv4Addr', () async {
      expect(await funcUsingIpv4Addr(arg: addr), addr);
    });
    test('funcUsingNonOpaqueStructContainingIpv4Addr', () async {
      final arg = NonOpaqueStructContainingIpv4Addr(inner: addr);
      expect(await funcUsingNonOpaqueStructContainingIpv4Addr(arg: arg), arg);
    });
  });
}
