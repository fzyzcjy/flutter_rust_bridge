import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  print('Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');

  var tt = Stopwatch()..start();
  await renderImage(width: 3000, height: 3000);
  print(' => IN FLUTTER: ${tt.elapsedMilliseconds} ms');
}
