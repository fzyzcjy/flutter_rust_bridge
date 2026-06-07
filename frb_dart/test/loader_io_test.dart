@TestOn('vm')
import 'package:flutter_rust_bridge/src/loader/_io.dart';
import 'package:test/test.dart';

void main() {
  test('Darwin packaged loader falls back to process after open failures', () {
    final attempts = <String>[];

    final result = loadDarwinPackagedExternalLibrary(
      stem: 'my_rust_lib',
      debugInfo: 'debug',
      open: (name, debugInfo) {
        attempts.add('open $name $debugInfo');
        throw ArgumentError('missing $name');
      },
      process: (debugInfo) {
        attempts.add('process $debugInfo');
        return 'ok';
      },
    );

    expect(result, 'ok');
    expect(attempts, [
      'open rust_builder.framework/rust_builder debug',
      'open my_rust_lib.framework/my_rust_lib debug (after trying rust_builder.framework/rust_builder but has error Invalid argument(s): missing rust_builder.framework/rust_builder)',
      'process debug (after trying rust_builder.framework/rust_builder but has error Invalid argument(s): missing rust_builder.framework/rust_builder) (after trying my_rust_lib.framework/my_rust_lib but has error Invalid argument(s): missing my_rust_lib.framework/my_rust_lib) (after falling back to process())',
    ]);
  });

  test('Darwin packaged loader does not hide non-argument failures', () {
    expect(
      () => loadDarwinPackagedExternalLibrary(
        stem: 'my_rust_lib',
        debugInfo: 'debug',
        open: (name, debugInfo) => throw StateError('unexpected'),
        // This callback must not be reached when the open failure is non-recoverable.
        // coverage:ignore-start
        process: (debugInfo) => 'ok',
        // coverage:ignore-end
      ),
      throwsStateError,
    );
  });
}
