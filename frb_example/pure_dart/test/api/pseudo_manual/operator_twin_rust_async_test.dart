// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `operator_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/operator_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('Rust operator traits become Dart operators', () {
    const a = OperatorPointTwinRustAsync(x: 2, y: 10);
    const b = OperatorPointTwinRustAsync(x: 2, y: 20);
    const c = OperatorPointTwinRustAsync(x: 3, y: 0);
    final sum = a + c;
    expect(sum.x, 5);
    expect(sum.y, 10);
    expect(a == b, isTrue);
    expect(a != b, isFalse);
    expect(a < c, isTrue);
    expect(a <= b, isTrue);
    expect(c > a, isTrue);
    expect(c >= a, isTrue);

    const six = OperatorValueTwinRustAsync(value: 6);
    const three = OperatorValueTwinRustAsync(value: 3);
    expect((six + three).value, 9);
    expect((six - three).value, 3);
    expect((six * three).value, 18);
    expect((six / three).value, 2);
    expect((six % OperatorValueTwinRustAsync(value: 4)).value, 2);
    expect((-six).value, -6);
    expect((~six).value, ~6);
    expect((six & three).value, 2);
    expect((six | three).value, 7);
    expect((six ^ three).value, 5);
    expect((six << 2).value, 24);
    expect((six >> 1).value, 3);
  });
}
