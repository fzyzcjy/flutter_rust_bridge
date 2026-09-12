import 'package:frb_example_dart_build_rs/src/rust/api/minimal.dart';
import 'package:frb_example_dart_build_rs/src/rust/frb_generated.dart';
import 'package:logging/logging.dart';
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

  test('build.rs bindings deliver Rust logs to Dart logging (fix #3402)',
      () async {
    const message = 'build.rs logging regression';
    final previousLevel = Logger.root.level;
    Logger.root.level = Level.ALL;
    addTearDown(() => Logger.root.level = previousLevel);

    final recordFuture = Logger.root.onRecord
        .firstWhere((record) => record.message == message)
        .timeout(const Duration(seconds: 10));

    await emitLogMessage(message: message);
    final record = await recordFuture;

    expect(record.message, message);
    expect(record.level, Level.WARNING);
    expect(record.loggerName, 'frb_build_rs_test');
  });
}
