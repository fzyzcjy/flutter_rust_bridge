import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  print('Action: Configure tests (end)');

  // ----------------------------------------------------------------

  test('empty', () async {
    final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
    expect(await ltGetAndResetLogsTwinNormal(), <String>[]);
    ownedStruct.dispose();
    expect(await ltGetAndResetLogsTwinNormal(),
        <String>['LtOwnedStructTwinNormal.drop']);
  });

  test('computeTypeWithLifetimeTwinNormal', () async {
    final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
    final typeWithLifetime =
        await ownedStruct.computeTypeWithLifetimeTwinNormal();

    ownedStruct.dispose();
    // Do *not* really dispose ownedStruct
    expect(await ltGetAndResetLogsTwinNormal(), <String>[]);

    expect(await typeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
    expect(await typeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');

    typeWithLifetime.dispose();
    expect(await ltGetAndResetLogsTwinNormal(), <String>[
      // NOTE order: Firstly the borrowed type, secondly the owned type
      'LtTypeWithLifetimeTwinNormal.drop',
      'LtOwnedStructTwinNormal.drop',
    ]);
  });
}
