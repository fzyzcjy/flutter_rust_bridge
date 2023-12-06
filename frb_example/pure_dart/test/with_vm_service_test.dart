import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
import 'package:frb_example_pure_dart/src/rust/api/dart_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/primitive_list_misc_twin_sync.dart';
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

  group('sync return', () {
    test('allocate a lot of zero copy data to check that it is properly freed',
        () async {
      const n = 10000;
      int calls = 0;

      expect(debugOnExternalTypedDataFinalizer, isNull);
      debugOnExternalTypedDataFinalizer = expectAsync1(
        (dataLength) {
          expect(dataLength, n);
          calls++;
        },
        count: 10,
        reason:
            "Finalizer must be called once for each returned packed primitive list",
      );
      addTearDown(() => debugOnExternalTypedDataFinalizer = null);

      ZeroCopyVecOfPrimitivePackTwinSync? primitivePack =
          handleZeroCopyVecOfPrimitiveTwinSync(n: n);
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
        print('hi dorptest 1');
        Uint8List? strongRef = createLargeList(mb: 300);
        print('hi dorptest 2');
        final weakRef = WeakReference(strongRef);
        print('hi dorptest 3');
        // TODO
        // TODO
        // TODO
        // await setStaticDartOpaqueTwinNormal(opaque: strongRef);
        (RustLib.instance.api as RustLibApiImpl)
            .generalizedFrbRustBinding
            .dartNewPersistentHandle(strongRef);
        print('hi dorptest 4');
        strongRef = null;

        // print('hi dorptest 5');
        // await vmService.gc();
        // print('hi dorptest 6');
        // await Future<void>.delayed(const Duration(milliseconds: 10));
        // print('hi dorptest 7');
        // expect(weakRef.target, isNotNull);
        //
        // print('hi dorptest 8');
        // TODO
        // TODO
        // TODO
        // await dropStaticDartOpaqueTwinNormal();
        // print('hi dorptest 9');
        // await vmService.gc();
        // print('hi dorptest 10');
        // await Future<void>.delayed(const Duration(milliseconds: 10));
        print('hi dorptest 11');
        expect(weakRef.target, isNull);
        print('hi dorptest 12');
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
}
