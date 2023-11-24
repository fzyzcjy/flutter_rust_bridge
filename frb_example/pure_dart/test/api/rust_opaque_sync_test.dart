import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('create', () {
    var data = syncCreateOpaque();
    data.dispose();
  });

  test('option', () async {
    var x = syncOptionRustOpaque();
    expect(x, isNotNull);
    x!.dispose();
  });

  test('nonclone', () async {
    var data = syncCreateNonClone();
    var data2 = await runNonClone(clone: data);
    expect(data2, "content");
    data.dispose();
  });

  test('double call', () {
    var data = syncCreateSyncOpaque();
    expect(
        syncRunOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        syncRunOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
  });

  test('call after drop', () {
    var data = syncCreateSyncOpaque();
    expect(
        syncRunOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    expect(() => syncRunOpaque(opaque: data), throwsA(isA<PanicException>()));
  });

  test('check generator', () {
    expect(frbSyncGeneratorTest().runtimeType == FrbOpaqueSyncReturn, isTrue);
  });
}
