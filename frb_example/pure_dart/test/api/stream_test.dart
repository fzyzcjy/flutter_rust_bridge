import 'package:frb_example_pure_dart/src/rust/api/stream.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call handleStreamRealisticTwinNormal', () async {
    final stream = handleStreamRealisticTwinNormal(arg: 'hello');
    var cnt = 0;
    await for (final value in stream) {
      debugPrint("output from handle_stream's stream: $value");
      cnt++;
    }
    expect(cnt, 10);
  });
}
