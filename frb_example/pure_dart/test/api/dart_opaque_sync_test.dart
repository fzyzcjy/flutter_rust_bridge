// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync"]}

import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();

  final binding = (RustLib.instance.api as RustLibApiImpl)
      .generalizedFrbRustBinding
      .binding;
  String f() => 'Test_String';
  final persistentHandle = binding.naive_NewPersistentHandle(f);
  binding.naive_HandleFromPersistent(persistentHandle);

  // String f() => 'Test_String';
  // var syncBack = syncLoopbackTwinNormal(opaque: f);
  // print('$f $syncBack');
}
