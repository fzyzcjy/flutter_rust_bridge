import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaque();
    var data = await createOpaque();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaque();
    var hideData = await runOpaque(opaque: opaque);

    expect(
        hideData,
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    opaque.dispose();
  });

  test('double Call', () async {
    var data = await createOpaque();
    expect(
        await runOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
  });

  test('call after dispose', () async {
    var data = await createOpaque();
    expect(
        await runOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectLater(() => runOpaque(opaque: data), throwsA(isA<PanicException>()));
  });

  test('dispose before complete', () async {
    var data = await createOpaque();
    var task = runOpaqueWithDelay(opaque: data);
    data.dispose();
    expect(
        await task,
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    await expectLater(() => runOpaque(opaque: data), throwsA(isA<PanicException>()));
  });

  test('create array of opaque type', () async {
    var data = await opaqueArray();
    for (var v in data) {
      expect(
          await runOpaque(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectLater(() => runOpaque(opaque: v), throwsA(isA<PanicException>()));
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnum();

    expect(
        await runEnumOpaque(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaque_Struct).field0.dispose();

    expect(await runEnumOpaque(opaque: data[1]), "42");
    (data[1] as EnumOpaque_Primitive).field0.dispose();

    expect(await runEnumOpaque(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaque_TraitObj).field0.dispose();

    expect(
        await runEnumOpaque(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaque_Mutex).field0.dispose();

    expect(
        await runEnumOpaque(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaque_RwLock).field0.dispose();
    await expectLater(() => runEnumOpaque(opaque: data[4]), throwsA(isA<PanicException>()));
  });

  test('opaque field', () async {
    var data = await createNestedOpaque();
    await runNestedOpaque(opaque: data);

    expect(
        await runOpaque(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaque(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectLater(() => runOpaque(opaque: data.first), throwsA(isA<PanicException>()));
    await expectLater(() => runNestedOpaque(opaque: data), throwsA(isA<PanicException>()));
    expect(
        await runOpaque(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.second.dispose();
  });

  test('array', () async {
    var data = await opaqueArray();
    await opaqueArrayRun(data: data);
    data[0].dispose();

    expect(
        await runOpaque(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueArrayRun(data: data), throwsA(isA<PanicException>()));
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVec();
    await opaqueVecRun(data: data);
    data[0].dispose();

    expect(
        await runOpaque(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueVecRun(data: data), throwsA(isA<PanicException>()));
    data[1].dispose();
  });

  test('unwrap', () async {
    var data = await createOpaque();
    data.move = true;
    expect(
        await unwrapRustOpaque(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(data.isStale(), isTrue);

    var data2 = await createOpaque();
    await expectLater(() => unwrapRustOpaque(opaque: data2), throwsA(isA<AnyhowException>()));
    expect(data2.isStale(), isFalse);
  });
}
