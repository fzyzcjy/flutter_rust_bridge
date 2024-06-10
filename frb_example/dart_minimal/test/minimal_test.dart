import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  print('Action: Configure tests (end)');

  test('func_stream_add_value_and_error_twin_normal', () async {
    final stream = await funcStreamAddValueAndErrorTwinNormal();
    final events = <String>[];
    final onDone = Completer<void>();
    stream.listen(
      (e) => events.add('data $e'),
      onError: (e, s) {
        print('onError $e $s');
        events.add('error $e');
      },
      onDone: () => onDone.complete(),
    );
    await onDone.future;
    expect(events, ['data 100', 'data 200', contains('deliberate error')]);
  });
}
