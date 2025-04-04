import 'package:frb_example_feat_no_dart_opaque/src/rust/api/minimal.dart';
import 'package:frb_example_feat_no_dart_opaque/src/rust/frb_generated.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  print('Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
