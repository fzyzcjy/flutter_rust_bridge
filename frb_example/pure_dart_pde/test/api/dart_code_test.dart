// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/dart_code.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('TranslatableStructWithDartCodeTwinNormal', () async {
    final one = TranslatableStructWithDartCodeTwinNormal(a: 100);
    final two = TranslatableStructWithDartCodeTwinNormal(a: 100);
    expect(one.hashCode, two.hashCode);
    expect(one == two, true);
    expect(one.dartCodeMethod(), 200);
    expect(await one.normalMethodTwinNormal(), 200);
  });

  test('OpaqueStructWithDartCodeTwinNormal', () async {
    expect(OpaqueStructWithDartCodeTwinNormal.dartCodeGetter, 123);
  });

  test('OpaqueStructWithDartCodeTwinNormal instance members', () async {
    final one = OpaqueStructWithDartCodeTwinNormal();
    final two = OpaqueStructWithDartCodeTwinNormal();
    // Two objects, so equality below comes from the override. hashCode => 7 and
    // universal == demonstrate an override; they are not a pattern to copy.
    expect(identical(one, two), false);
    expect(await one.dartCodeMethod(), 200);
    expect(one.toString(), 'OpaqueStructWithDartCodeTwinNormal(dart_code)');
    expect(one.hashCode, 7);
    expect(one == two, true);
    expect({one, two}.length, 1);

    one.dispose();
    expect(one.isDisposed, true);
  });

  test('ProxiedOpaqueStructWithDartCodeTwinNormal instance members', () async {
    final owner = ProxiedOpaqueStructWithDartCodeOwnerTwinNormal();
    final inner = await owner.innerTwinNormal();
    expect(await inner.dartCodeMethod(), 200);
    expect(inner.toString(),
        'ProxiedOpaqueStructWithDartCodeTwinNormal(dart_code)');

    // SimpleDisposable.dispose only flips this flag. It does not release the
    // upstream Rust object.
    inner.dispose();
    expect(inner.isDisposed, true);
  });
}
