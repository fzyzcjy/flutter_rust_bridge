import 'dart:math';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/dropping.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import 'test_utils.dart';
import 'utils/test_flutter_memory_leak_utility.dart';

Future<void> main() async {
  await RustLib.init();

  final vmService = await VmServiceUtil.create();
  if (vmService == null) {
    // Related: https://github.com/dart-lang/sdk/issues/54155
    fail(
        'To run these tests, you should enable VM service like: `dart --enable-vm-service test`.');
  }
  tearDownAll(() => vmService.dispose());

  // sync zero copy is temporarily disabled to let VM understand external size
  // group('sync return', () {
  //   test('allocate a lot of zero copy data to check that it is properly freed',
  //       () async {
  //     const n = 10000;
  //     int calls = 0;
  //
  //     expect(debugOnExternalTypedDataFinalizer, isNull);
  //     debugOnExternalTypedDataFinalizer = expectAsync1(
  //       (dataLength) {
  //         expect(dataLength, n);
  //         calls++;
  //       },
  //       count: 10,
  //       reason:
  //           "Finalizer must be called once for each returned packed primitive list",
  //     );
  //     addTearDown(() => debugOnExternalTypedDataFinalizer = null);
  //
  //     // it is auto zero-copied
  //     VecOfPrimitivePackTwinSync? primitivePack =
  //         handleVecOfPrimitiveTwinSync(n: n);
  //     await vmService.gc();
  //     await Future<void>.delayed(const Duration(milliseconds: 10));
  //     expect(primitivePack, isNotNull);
  //     expect(calls, 0);
  //
  //     primitivePack = null;
  //     await vmService.gc();
  //     await Future<void>.delayed(const Duration(milliseconds: 10));
  //   });
  // });

  group('dart opaque type', () {
    group('GC', () {
      test('drop', () async {
        final id = Random().nextInt(1000000000);

        // NOTE: If large list + create dart persistent handle,
        // then the `weakRef.target` will run for a long time, and it seems
        // this time is related to the Uint8List size here.
        // So we make a small list here.
        // https://github.com/fzyzcjy/yplusplus/issues/11352#issuecomment-1841943168
        // Uint8List? strongRef = createLargeList(mb: 300);
        Uint8List? strongRef = Uint8List(10000);
        final weakRef = WeakReference(strongRef);
        await setStaticDartOpaqueTwinNormal(id: id, opaque: strongRef);
        strongRef = null;

        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
        expect(weakRef.target, isNotNull);

        await dropStaticDartOpaqueTwinNormal(id: id);

        final anotherWeakRef = WeakReference(createLargeList(mb: 300));

        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));

        expect(anotherWeakRef.target, isNull,
            reason: 'to ensure GC is triggered');
        expect(weakRef.target, isNull);
      });

      test('unwrap', () async {
        Uint8List? strongRef = createLargeList(mb: 300);
        final weakRef = WeakReference(strongRef);
        expect(unwrapDartOpaqueTwinNormal(opaque: strongRef), 'Test');
        strongRef = null;

        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
        expect(weakRef.target, isNull);
      });
    });
  });

  group('Rust object is dropped', () {
    for (final dropApproach in _DropApproach.values) {
      group('dropApproach=$dropApproach', () {
        Future<void> _core({
          Future<void> Function(DroppableTwinNormal)? extra,
        }) async {
          DroppableTwinNormal? object =
              await DroppableTwinNormal.newTwinNormal();
          final weakRef = WeakReference(object);

          await extra?.call(object);

          final oldDropCount =
              await DroppableTwinNormal.getDropCountTwinNormal();

          switch (dropApproach) {
            case _DropApproach.auto:
              object = null;
              await vmService.gc();
              expect(object, null);
              expect(weakRef.target, null);

            case _DropApproach.manual:
              object.dispose();
              expect(object, isNotNull);
          }

          expect(await DroppableTwinNormal.getDropCountTwinNormal(),
              oldDropCount + 1);
        }

        test('Rust object should be dropped', () async {
          await _core();
        });

        // #1723
        test('when holds StreamSink, Rust object should be dropped', () async {
          await _core(
              extra: (object) async => object.createStream().listen((_) {}));
        });
      });
    }
  });
}

enum _DropApproach { auto, manual }
