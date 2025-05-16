// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/impl_trait_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('impl class should extend trait class', () async {
    final one =
        await StructOneWithTraitTwinSyncSse.simpleTraitFnTwinSyncSse(value: 10);
    expect(one, isA<SimpleTraitTwinSyncSse>());
  });

  test('call methods', () async {
    final one =
        await StructOneWithTraitTwinSyncSse.simpleTraitFnTwinSyncSse(value: 10);
    expect(await one.simpleTraitFnReceiverBorrowTwinSyncSse(), 10);
  });

  test('trait default impl', () async {
    expect(
        await StructOneWithTraitTwinSyncSse
            .simpleTraitFnWithDefaultImplTwinSyncSse(),
        42);
  });

  // test('use generated implementor', () async {
  //   final object =
  //       await StructOneWithTraitForDynTwinSyncSse.createTwinSyncSse(one: 100);
  //   expect(
  //       await funcArgTraitImplTwinSyncSse(
  //           arg: SimpleTraitForDynTwinSyncSseImplementor
  //               .structOneWithTraitForDynTwinSyncSse(object)),
  //       100);
  // });
}
