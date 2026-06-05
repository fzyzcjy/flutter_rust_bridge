import 'package:flutter_rust_bridge/src/loader/_io.dart';
import 'package:test/test.dart';

void main() {
  test('Darwin packaged loader falls back to process after open failures', () {
    final attempts = <String>[];

    final result = loadDarwinPackagedExternalLibraryForTesting(
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
}
