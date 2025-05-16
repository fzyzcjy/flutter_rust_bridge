// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_rust_async_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaqueTwinRustAsyncMoi();
    var data = await createOpaqueTwinRustAsyncMoi();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaqueTwinRustAsyncMoi();
    var hideData = await runOpaqueTwinRustAsyncMoi(opaque: opaque);

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
    var data = await createOpaqueTwinRustAsyncMoi();
    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data),
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
    var data = await createOpaqueTwinRustAsyncMoi();
    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectLater(() => runOpaqueTwinRustAsyncMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('dispose before complete', () async {
    var data = await createOpaqueTwinRustAsyncMoi();
    var task = runOpaqueWithDelayTwinRustAsyncMoi(opaque: data);
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
    await expectLater(() => runOpaqueTwinRustAsyncMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('create array of opaque type', () async {
    var data = await opaqueArrayTwinRustAsyncMoi();
    for (var v in data) {
      expect(
          await runOpaqueTwinRustAsyncMoi(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectLater(() => runOpaqueTwinRustAsyncMoi(opaque: v),
          throwsA(isA<DroppableDisposedException>()));
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnumTwinRustAsyncMoi();

    expect(
        await runEnumOpaqueTwinRustAsyncMoi(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaqueTwinRustAsyncMoi_Struct).field0.dispose();

    expect(await runEnumOpaqueTwinRustAsyncMoi(opaque: data[1]), "42");
    (data[1] as EnumOpaqueTwinRustAsyncMoi_Primitive).field0.dispose();

    expect(await runEnumOpaqueTwinRustAsyncMoi(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaqueTwinRustAsyncMoi_TraitObj).field0.dispose();

    expect(
        await runEnumOpaqueTwinRustAsyncMoi(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaqueTwinRustAsyncMoi_Mutex).field0.dispose();

    expect(
        await runEnumOpaqueTwinRustAsyncMoi(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaqueTwinRustAsyncMoi_RwLock).field0.dispose();
    await expectLater(() => runEnumOpaqueTwinRustAsyncMoi(opaque: data[4]),
        throwsA(isA<DroppableDisposedException>()));
  });

  test('opaque field', () async {
    var data = await createNestedOpaqueTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        runNestedOpaqueTwinRustAsyncMoi(opaque: data));

    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectLater(() => runOpaqueTwinRustAsyncMoi(opaque: data.first),
        throwsA(isA<DroppableDisposedException>()));
    await expectLater(() => runNestedOpaqueTwinRustAsyncMoi(opaque: data),
        throwsA(isA<DroppableDisposedException>()));
    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data.second),
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
    var data = await opaqueArrayTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        opaqueArrayRunTwinRustAsyncMoi(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueArrayRunTwinRustAsyncMoi(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVecTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        opaqueVecRunTwinRustAsyncMoi(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinRustAsyncMoi(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectLater(() => opaqueVecRunTwinRustAsyncMoi(data: data),
        throwsA(isA<DroppableDisposedException>()));
    data[1].dispose();
  });

  // test('unwrap', () async {
  //   var data = await createOpaqueTwinRustAsyncMoi();
  //   data.move = true;
  //   expect(
  //       await unwrapRustOpaqueTwinRustAsyncMoi(opaque: data),
  //       "content - Some(PrivateData "
  //       "{"
  //       " content: \"content nested\", "
  //       "primitive: 424242, "
  //       "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
  //       "lifetime: \"static str\" "
  //       "})");
  //   expect(data.isDisposed, isTrue);
  //
  //   var data2 = await createOpaqueTwinRustAsyncMoi();
  //   await expectLater(() => unwrapRustOpaqueTwinRustAsyncMoi(opaque: data2),
  //       throwsA(isA<AnyhowException>()));
  //   expect(data2.isDisposed, isFalse);
  // });
}
