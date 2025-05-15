import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  // The `TypeForIgnore` type has a single field called `field_1` which the
  // constructor always sets to 0.  The field is marked as `#[frb(ignore)]`
  // which means that the FRB code generater should not generate automatic
  // accessors for it.
  //
  // To ensure that no accessors were generated the type also has a `field_1`
  // method which always returns 1.
  //
  // - If the attribute isn't recognized then FRB codegen will generate an
  //   automatic accessor and we'll get a compile error due to the `field_1`
  //   accessor conflict.
  // - If the the automatic accessor ends up taking precedence then the
  //   following will fail to compile because the accessor is called as a
  //   function.
  test('make sure #[frb(ignore)] works', () async {
    var t = await TypeForIgnore.newInstance();
    expect(await t.field1(), 1);
  });
}
