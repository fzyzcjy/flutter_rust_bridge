// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('create and dispose', () async {
    var futureData = createOpaqueTwinSync();
    var data = await createOpaqueTwinSync();
    data.dispose();
    (await futureData).dispose();
  });

  test('simple call', () async {
    var opaque = await createOpaqueTwinSync();
    var hideData = await runOpaqueTwinSync(opaque: opaque);

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
    var data = await createOpaqueTwinSync();
    expect(
        await runOpaqueTwinSync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinSync(opaque: data),
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
    var data = await createOpaqueTwinSync();
    expect(
        await runOpaqueTwinSync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.dispose();
    await expectRustPanic(() => runOpaqueTwinSync(opaque: data), 'TwinSync');
  });

  test('dispose before complete', () async {
    var data = await createOpaqueTwinSync();
    var task = runOpaqueWithDelayTwinSync(opaque: data);
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
    await expectRustPanic(() => runOpaqueTwinSync(opaque: data), 'TwinSync');
  });

  test('create array of opaque type', () async {
    var data = await opaqueArrayTwinSync();
    for (var v in data) {
      expect(
          await runOpaqueTwinSync(opaque: v),
          "content - Some(PrivateData "
          "{"
          " content: \"content nested\", "
          "primitive: 424242, "
          "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
          "lifetime: \"static str\" "
          "})");
      v.dispose();
      await expectRustPanic(() => runOpaqueTwinSync(opaque: v), 'TwinSync');
    }
  });

  test('create enums of opaque type', () async {
    var data = await createArrayOpaqueEnumTwinSync();

    expect(
        await runEnumOpaqueTwinSync(opaque: data[0]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    (data[0] as EnumOpaqueTwinSync_Struct).field0.dispose();

    expect(await runEnumOpaqueTwinSync(opaque: data[1]), "42");
    (data[1] as EnumOpaqueTwinSync_Primitive).field0.dispose();

    expect(await runEnumOpaqueTwinSync(opaque: data[2]), "\"String\"");
    (data[2] as EnumOpaqueTwinSync_TraitObj).field0.dispose();

    expect(
        await runEnumOpaqueTwinSync(opaque: data[3]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[3] as EnumOpaqueTwinSync_Mutex).field0.dispose();

    expect(
        await runEnumOpaqueTwinSync(opaque: data[4]),
        "\"content - Some(PrivateData "
        "{"
        " content: \\\"content nested\\\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \\\"static str\\\" "
        "})\"");
    (data[4] as EnumOpaqueTwinSync_RwLock).field0.dispose();
    await expectRustPanic(
        () => runEnumOpaqueTwinSync(opaque: data[4]), 'TwinSync');
  });

  test('opaque field', () async {
    var data = await createNestedOpaqueTwinSync();
    await futurizeVoidTwinSync(runNestedOpaqueTwinSync(opaque: data));

    expect(
        await runOpaqueTwinSync(opaque: data.first),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(
        await runOpaqueTwinSync(opaque: data.second),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    data.first.dispose();
    await expectRustPanic(
        () => runOpaqueTwinSync(opaque: data.first), 'TwinSync');
    await expectRustPanic(
        () => runNestedOpaqueTwinSync(opaque: data), 'TwinSync');
    expect(
        await runOpaqueTwinSync(opaque: data.second),
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
    var data = await opaqueArrayTwinSync();
    await futurizeVoidTwinSync(opaqueArrayRunTwinSync(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinSync(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectRustPanic(() => opaqueArrayRunTwinSync(data: data), 'TwinSync');
    data[1].dispose();
  });

  test('vec', () async {
    var data = await opaqueVecTwinSync();
    await futurizeVoidTwinSync(opaqueVecRunTwinSync(data: data));
    data[0].dispose();

    expect(
        await runOpaqueTwinSync(opaque: data[1]),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");

    await expectRustPanic(() => opaqueVecRunTwinSync(data: data), 'TwinSync');
    data[1].dispose();
  });

  test('unwrap', () async {
    var data = await createOpaqueTwinSync();
    data.move = true;
    expect(
        await unwrapRustOpaqueTwinSync(opaque: data),
        "content - Some(PrivateData "
        "{"
        " content: \"content nested\", "
        "primitive: 424242, "
        "array: [451, 451, 451, 451, 451, 451, 451, 451, 451, 451], "
        "lifetime: \"static str\" "
        "})");
    expect(data.isDisposed, isTrue);

    var data2 = await createOpaqueTwinSync();
    await expectLater(() => unwrapRustOpaqueTwinSync(opaque: data2),
        throwsA(isA<AnyhowException>()));
    expect(data2.isDisposed, isFalse);
  });
}
