// ignore_for_file: invalid_use_of_internal_member

@TestOn('vm')
import 'package:flutter_rust_bridge/src/generalized_uint8list/rust_vec_u8.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('simple RustVecU8', () async {
    final vec = RustVecU8(
        10, (RustLib.instance.api as RustLibApiImpl).generalizedFrbRustBinding);
    vec[0] = 42;
    vec.dispose();
  });
}
