// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync"]}

import 'package:frb_example_pure_dart/src/rust/api/dart_opaque_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  String f() => 'Test_String';
  var syncBack = syncLoopbackTwinNormal(opaque: f);
  print('$f $syncBack');
}
