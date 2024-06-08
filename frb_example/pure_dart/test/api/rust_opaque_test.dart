// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaqueTwinNormal();
    var data = await createOpaqueTwinNormal();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaqueTwinNormal();
    var hideData = await runOpaqueTwinNormal(opaque: opaque);

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
    var data = await createOpaqueTwinNormal();
    expect(
        await runOpaqueTwinNormal(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinNormal(opaque: data),
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
    var data = await createOpaqueTwinNormal();
    expect(
        await runOpaqueTwinNormal(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectLater(() => runOpaqueTwinNormal(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('dispose before complete', () async {
    var data = await createOpaqueTwinNormal();
    var task = runOpaqueWithDelayTwinNormal(opaque: data);
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
    await expectLater(() => runOpaqueTwinNormal(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('create array of opaque type', () async {
    var data = await opaqueArrayTwinNormal();
    for (var v in data) {
      expect(
          await runOpaqueTwinNormal(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectLater(() => runOpaqueTwinNormal(opaque: v),
          throwsA(isA<DroppableDisposedException>()));
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnumTwinNormal();

    expect(
        await runEnumOpaqueTwinNormal(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaqueTwinNormal_Struct).field0.dispose();

    expect(await runEnumOpaqueTwinNormal(opaque: data[1]), "42");
    (data[1] as EnumOpaqueTwinNormal_Primitive).field0.dispose();

    expect(await runEnumOpaqueTwinNormal(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaqueTwinNormal_TraitObj).field0.dispose();

    expect(
        await runEnumOpaqueTwinNormal(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaqueTwinNormal_Mutex).field0.dispose();

    expect(
        await runEnumOpaqueTwinNormal(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaqueTwinNormal_RwLock).field0.dispose();
    await expectLater(() => runEnumOpaqueTwinNormal(opaque: data[4]),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('opaque field', () async {
    var data = await createNestedOpaqueTwinNormal();
    await futurizeVoidTwinNormal(runNestedOpaqueTwinNormal(opaque: data));

    expect(
        await runOpaqueTwinNormal(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinNormal(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectLater(() => runOpaqueTwinNormal(opaque: data.first),
        throwsA(isA<DroppableDisposedException>()));
    await expectLater(() => runNestedOpaqueTwinNormal(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
    expect(
        await runOpaqueTwinNormal(opaque: data.second),
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
    var data = await opaqueArrayTwinNormal();
    await futurizeVoidTwinNormal(opaqueArrayRunTwinNormal(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinNormal(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueArrayRunTwinNormal(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVecTwinNormal();
    await futurizeVoidTwinNormal(opaqueVecRunTwinNormal(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinNormal(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueVecRunTwinNormal(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  // test('unwrap', () async {
  //   var data = await createOpaqueTwinNormal();
  //   data.move = true;
  //   expect(
  //       await unwrapRustOpaqueTwinNormal(opaque: data),
  //       "content - Some(PrivateData "
  //       "{"
  //       " content: \"content nested\", "
  //       "primitive: 424242, "
  //       "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
  //       "lifetime: \"static str\" "
  //       "})");
  //   expect(data.isDisposed, isTrue);
  //
  //   var data2 = await createOpaqueTwinNormal();
  //   await expectLater(() => unwrapRustOpaqueTwinNormal(opaque: data2),
  //       throwsA(isA<AnyhowException>()));
  //   expect(data2.isDisposed, isFalse);
  // });
}
