import 'package:frb_example_pure_dart/src/rust/api/impl_trait.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
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
    expect(one.simpleTraitFnReceiverBorrowTwinNormal(), 10);
  });

  test('trait default impl', () async {
    expect(
        await StructOneWithTraitTwinNormal
            .simpleTraitFnWithDefaultImplTwinNormal(),
        42);
  });
}