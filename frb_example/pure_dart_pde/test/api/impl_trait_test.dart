// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/impl_trait.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('impl class should extend trait class', () async {
    final one =
        await StructOneWithTraitTwinNormal.simpleTraitFnTwinNormal(value: 10);
    expect(one, isA<SimpleTraitTwinNormal>());
  });

  test('call methods', () async {
    final one =
        await StructOneWithTraitTwinNormal.simpleTraitFnTwinNormal(value: 10);
    expect(await one.simpleTraitFnReceiverBorrowTwinNormal(), 10);
  });

  test('trait default impl', () async {
    expect(
        await StructOneWithTraitTwinNormal
            .simpleTraitFnWithDefaultImplTwinNormal(),
        42);
  });

  // test('use generated implementor', () async {
  //   final object =
  //       await StructOneWithTraitForDynTwinNormal.createTwinNormal(one: 100);
  //   expect(
  //       await funcArgTraitImplTwinNormal(
  //           arg: SimpleTraitForDynTwinNormalImplementor
  //               .structOneWithTraitForDynTwinNormal(object)),
  //       100);
  // });
}
