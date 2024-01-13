// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_example_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/misc_type_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/optional_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/auxiliary/sample_types.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('call funcReturnUnitTwinSyncSse', () async {
    // `as dynamic` to make the generated pseudo_manual test happy
    await (funcReturnUnitTwinSyncSse() as dynamic);
  });

  test('dart call handleListOfStruct', () async {
    final listOfStructResp = await handleListOfStructTwinSyncSse(
        l: [MySize(width: 42, height: 100), MySize(width: 420, height: 1000)]);
    expect(listOfStructResp.length, 4);
    expect(listOfStructResp[0].width, 42);
    expect(listOfStructResp[1].width, 420);
    expect(listOfStructResp[2].width, 42);
    expect(listOfStructResp[3].width, 420);
  });

  test('dart call handleStringList', () async {
    final names =
        await handleStringListTwinSyncSse(names: ['Steve', 'Bob', 'Alex']);
    expect(names, ['Steve', 'Bob', 'Alex']);
  });

  test('dart call handleVecOfOpts', () async {
    const loops = 20;
    var opt = OptVecsTwinSyncSse(
        i32: [],
        enums: [WeekdaysTwinSyncSse.monday],
        strings: ['foo'],
        buffers: []);
    for (var i = 0; i < loops; i++) {
      opt = await handleVecOfOptsTwinSyncSse(opt: opt);
    }
    final nulls = List.filled(loops, null);
    expect(opt.i32, nulls);
    expect(
        opt.enums, [WeekdaysTwinSyncSse.monday, for (final val in nulls) val]);
    expect(opt.strings, ['foo', for (final val in nulls) val]);
    expect(opt.buffers, nulls);
  });

  test('test empty struct', () async {
    final empty = EmptyTwinSyncSse();
    final output = await emptyStructTwinSyncSse(empty: empty);
    expect(output, isA<EmptyTwinSyncSse>());
  });
}
