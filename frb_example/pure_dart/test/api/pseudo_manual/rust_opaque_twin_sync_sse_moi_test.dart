// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_sync_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaqueTwinSyncSseMoi();
    var data = await createOpaqueTwinSyncSseMoi();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaqueTwinSyncSseMoi();
    var hideData = await runOpaqueTwinSyncSseMoi(opaque: opaque);

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
    var data = await createOpaqueTwinSyncSseMoi();
    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data),
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
    var data = await createOpaqueTwinSyncSseMoi();
    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectLater(() => runOpaqueTwinSyncSseMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('dispose before complete', () async {
    var data = await createOpaqueTwinSyncSseMoi();
    var task = runOpaqueWithDelayTwinSyncSseMoi(opaque: data);
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
    await expectLater(() => runOpaqueTwinSyncSseMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('create array of opaque type', () async {
    var data = await opaqueArrayTwinSyncSseMoi();
    for (var v in data) {
      expect(
          await runOpaqueTwinSyncSseMoi(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectLater(() => runOpaqueTwinSyncSseMoi(opaque: v),
          throwsA(isA<DroppableDisposedException>()));
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnumTwinSyncSseMoi();

    expect(
        await runEnumOpaqueTwinSyncSseMoi(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaqueTwinSyncSseMoi_Struct).field0.dispose();

    expect(await runEnumOpaqueTwinSyncSseMoi(opaque: data[1]), "42");
    (data[1] as EnumOpaqueTwinSyncSseMoi_Primitive).field0.dispose();

    expect(await runEnumOpaqueTwinSyncSseMoi(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaqueTwinSyncSseMoi_TraitObj).field0.dispose();

    expect(
        await runEnumOpaqueTwinSyncSseMoi(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaqueTwinSyncSseMoi_Mutex).field0.dispose();

    expect(
        await runEnumOpaqueTwinSyncSseMoi(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaqueTwinSyncSseMoi_RwLock).field0.dispose();
    await expectLater(() => runEnumOpaqueTwinSyncSseMoi(opaque: data[4]),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('opaque field', () async {
    var data = await createNestedOpaqueTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(
        runNestedOpaqueTwinSyncSseMoi(opaque: data));

    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectLater(() => runOpaqueTwinSyncSseMoi(opaque: data.first),
        throwsA(isA<DroppableDisposedException>()));
    await expectLater(() => runNestedOpaqueTwinSyncSseMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data.second),
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
    var data = await opaqueArrayTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(opaqueArrayRunTwinSyncSseMoi(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueArrayRunTwinSyncSseMoi(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVecTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(opaqueVecRunTwinSyncSseMoi(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinSyncSseMoi(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueVecRunTwinSyncSseMoi(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  // test('unwrap', () async {
  //   var data = await createOpaqueTwinSyncSseMoi();
  //   data.move = true;
  //   expect(
  //       await unwrapRustOpaqueTwinSyncSseMoi(opaque: data),
  //       "content - Some(PrivateData "
  //       "{"
  //       " content: \"content nested\", "
  //       "primitive: 424242, "
  //       "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
  //       "lifetime: \"static str\" "
  //       "})");
  //   expect(data.isDisposed, isTrue);
  //
  //   var data2 = await createOpaqueTwinSyncSseMoi();
  //   await expectLater(() => unwrapRustOpaqueTwinSyncSseMoi(opaque: data2),
  //       throwsA(isA<AnyhowException>()));
  //   expect(data2.isDisposed, isFalse);
  // });
}
