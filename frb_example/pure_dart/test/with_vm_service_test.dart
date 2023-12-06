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
    // print('hi dorptest 4');
    // strongRef = null;
    //
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
}
