import 'dart:async';
import 'dart:io';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  // print('Action: Configure tests (before)');
  // test('dart call minimalAdder', () async {
  //   print('Action: Call rust (before)');
  //   expect(await minimalAdder(a: 100, b: 200), 300);
  //   print('Action: Call rust (after)');
  // });
  // print('Action: Configure tests (end)');

  final addr = InternetAddress.tryParse('192.168.0.1')!;
  test('funcUsingIpv4Addr', () async {
    expect(await funcUsingIpv4Addr(arg: addr), addr);
  });
  test('funcUsingNonOpaqueStructContainingIpv4Addr', () async {
    final arg = NonOpaqueStructContainingIpv4Addr(inner: addr);
    expect(await funcUsingNonOpaqueStructContainingIpv4Addr(arg: arg), arg);
  });
}
