// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `impl_trait_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/impl_trait_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('impl class should extend trait class', () async {
    final one =
        await StructOneWithTraitTwinSync.simpleTraitFnTwinSync(value: 10);
    expect(one, isA<SimpleTraitTwinSync>());
  });

  test('call methods', () async {
    final one =
        await StructOneWithTraitTwinSync.simpleTraitFnTwinSync(value: 10);
    expect(await one.simpleTraitFnReceiverBorrowTwinSync(), 10);
  });

  test('trait default impl', () async {
    expect(
        await StructOneWithTraitTwinSync.simpleTraitFnWithDefaultImplTwinSync(),
        42);
  });

  // test('use generated implementor', () async {
  //   final object =
  //       await StructOneWithTraitForDynTwinSync.createTwinSync(one: 100);
  //   expect(
  //       await funcArgTraitImplTwinSync(
  //           arg: SimpleTraitForDynTwinSyncImplementor
  //               .structOneWithTraitForDynTwinSync(object)),
  //       100);
  // });
}
