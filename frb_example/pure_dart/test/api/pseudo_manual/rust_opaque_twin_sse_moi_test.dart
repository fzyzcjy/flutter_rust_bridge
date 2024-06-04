// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaqueTwinSseMoi();
    var data = await createOpaqueTwinSseMoi();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaqueTwinSseMoi();
    var hideData = await runOpaqueTwinSseMoi(opaque: opaque);

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
    var data = await createOpaqueTwinSseMoi();
    expect(
        await runOpaqueTwinSseMoi(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinSseMoi(opaque: data),
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
    var data = await createOpaqueTwinSseMoi();
    expect(
        await runOpaqueTwinSseMoi(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectLater(() => runOpaqueTwinSseMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('dispose before complete', () async {
    var data = await createOpaqueTwinSseMoi();
    var task = runOpaqueWithDelayTwinSseMoi(opaque: data);
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
    await expectLater(() => runOpaqueTwinSseMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('create array of opaque type', () async {
    var data = await opaqueArrayTwinSseMoi();
    for (var v in data) {
      expect(
          await runOpaqueTwinSseMoi(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectLater(() => runOpaqueTwinSseMoi(opaque: v),
          throwsA(isA<DroppableDisposedException>()));
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnumTwinSseMoi();

    expect(
        await runEnumOpaqueTwinSseMoi(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaqueTwinSseMoi_Struct).field0.dispose();

    expect(await runEnumOpaqueTwinSseMoi(opaque: data[1]), "42");
    (data[1] as EnumOpaqueTwinSseMoi_Primitive).field0.dispose();

    expect(await runEnumOpaqueTwinSseMoi(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaqueTwinSseMoi_TraitObj).field0.dispose();

    expect(
        await runEnumOpaqueTwinSseMoi(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaqueTwinSseMoi_Mutex).field0.dispose();

    expect(
        await runEnumOpaqueTwinSseMoi(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaqueTwinSseMoi_RwLock).field0.dispose();
    await expectLater(() => runEnumOpaqueTwinSseMoi(opaque: data[4]),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('opaque field', () async {
    var data = await createNestedOpaqueTwinSseMoi();
    await futurizeVoidTwinSseMoi(runNestedOpaqueTwinSseMoi(opaque: data));

    expect(
        await runOpaqueTwinSseMoi(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinSseMoi(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectLater(() => runOpaqueTwinSseMoi(opaque: data.first),
        throwsA(isA<DroppableDisposedException>()));
    await expectLater(() => runNestedOpaqueTwinSseMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
    expect(
        await runOpaqueTwinSseMoi(opaque: data.second),
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
    var data = await opaqueArrayTwinSseMoi();
    await futurizeVoidTwinSseMoi(opaqueArrayRunTwinSseMoi(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinSseMoi(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueArrayRunTwinSseMoi(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVecTwinSseMoi();
    await futurizeVoidTwinSseMoi(opaqueVecRunTwinSseMoi(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinSseMoi(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueVecRunTwinSseMoi(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  // test('unwrap', () async {
  //   var data = await createOpaqueTwinSseMoi();
  //   data.move = true;
  //   expect(
  //       await unwrapRustOpaqueTwinSseMoi(opaque: data),
  //       "content - Some(PrivateData "
  //       "{"
  //       " content: \"content nested\", "
  //       "primitive: 424242, "
  //       "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
  //       "lifetime: \"static str\" "
  //       "})");
  //   expect(data.isDisposed, isTrue);
  //
  //   var data2 = await createOpaqueTwinSseMoi();
  //   await expectLater(() => unwrapRustOpaqueTwinSseMoi(opaque: data2),
  //       throwsA(isA<AnyhowException>()));
  //   expect(data2.isDisposed, isFalse);
  // });
}
