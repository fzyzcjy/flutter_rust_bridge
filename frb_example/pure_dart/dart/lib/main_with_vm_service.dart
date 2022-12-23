import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
// ignore: implementation_imports
import 'package:flutter_rust_bridge/src/ffi/dart_cobject.dart' as dart_cobject;
import 'package:flutter_rust_bridge_example/bridge_definitions.dart';
import 'package:test/test.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'main.dart';
import 'test_flutter_memory_leak_utility.dart';

const isWeb = bool.fromEnvironment('dart.library.html');

String? skipWeb([String reason = 'unspecified']) => isWeb ? 'Skipped on web (reason: $reason)' : null;

void main(List<String> args) async {
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);
  final vmService = await VmServiceUtil.create();
  tearDownAll(() {
    api.dispose();
    vmService.dispose();
  });

  group('sync return', () {
    test('allocate a lot of zero copy data to check that it is properly freed', () async {
      const n = 10000;
      int calls = 0;
      dart_cobject.testTool!.onExternalTypedDataFinalizer.add(expectAsync1(
        (length) {
          expect(length, n);
          calls++;
        },
        count: 10,
        reason: "Finalizer must be called once for each returned packed primitive list",
      ));

      ZeroCopyVecOfPrimitivePack? primitivePack = api.handleZeroCopyVecOfPrimitiveSync(n: n);
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
        Uint8List? strongRef = createLargeList(mb: 300);
        final weakRef = WeakReference(strongRef);
        await api.setStaticDartOpaque(opaque: strongRef);
        strongRef = null;

        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
        expect(weakRef.target, isNotNull);

        await api.dropStaticDartOpaque();
        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
        expect(weakRef.target, isNull);
      });

      test('unwrap', () async {
        Uint8List? strongRef = createLargeList(mb: 300);
        final weakRef = WeakReference(strongRef);
        expect(api.unwrapDartOpaque(opaque: strongRef), 'Test');
        strongRef = null;

        await vmService.gc();
        await Future<void>.delayed(const Duration(milliseconds: 10));
        expect(weakRef.target, isNull);
      });
    });
  });
}
