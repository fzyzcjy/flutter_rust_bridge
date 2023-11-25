import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/primitive_list_misc.dart';
import 'package:frb_example_pure_dart/src/rust/api/primitive_list_sync_misc.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import 'test_utils.dart';
import 'utils/test_flutter_memory_leak_utility.dart';

Future<void> main() async {
  await RustLib.init();

  final maybeVmService = await VmServiceUtil.create();
  tearDownAll(() => maybeVmService?.dispose());

  group('tests with vm service',
      skip: maybeVmService == null ? 'The tests in this file is skipped, because it needs VMService to run.' : null,
      () {
    group('sync return', () {
      test('allocate a lot of zero copy data to check that it is properly freed', () async {
        final vmService = maybeVmService!;

        const n = 10000;
        int calls = 0;

        expect(debugOnExternalTypedDataFinalizer, isNull);
        debugOnExternalTypedDataFinalizer = expectAsync1(
          (dataLength) {
            expect(dataLength, n);
            calls++;
          },
          count: 10,
          reason: "Finalizer must be called once for each returned packed primitive list",
        );
        addTearDown(() => debugOnExternalTypedDataFinalizer = null);

        ZeroCopyVecOfPrimitivePack? primitivePack = handleZeroCopyVecOfPrimitiveSync(n: n);
        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
        expect(primitivePack, isNotNull);
        expect(calls, 0);

        primitivePack = null;
        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
      });
    });

    group('dart opaque type', () {
      group('GC', () {
        test('drop', () async {
          final vmService = maybeVmService!;

          Uint8List? strongRef = createLargeList(mb: 300);
          final weakRef = WeakReference(strongRef);
          await setStaticDartOpaque(opaque: strongRef);
          strongRef = null;

          await vmService.gc();
          await Future<void>.delayed(const Duration(milliseconds: 10));
          expect(weakRef.target, isNotNull);

          await dropStaticDartOpaque();
          await vmService.gc();
          await Future<void>.delayed(const Duration(milliseconds: 10));
          expect(weakRef.target, isNull);
        });

        test('unwrap', () async {
          final vmService = maybeVmService!;

          Uint8List? strongRef = createLargeList(mb: 300);
          final weakRef = WeakReference(strongRef);
          expect(unwrapDartOpaque(opaque: strongRef), 'Test');
          strongRef = null;

          await vmService.gc();
          await Future<void>.delayed(const Duration(milliseconds: 10));
          expect(weakRef.target, isNull);
        });
      });
    });
  });
}
