import 'dart:typed_data';

import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils_web.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';

const bufferLength = 64 * 1024 * 1024;

Future<void> main() async {
  await dartWebTestEntrypoint(() async {
    await RustLib.init();

    await processLargeBytes(bytes: Uint8List(1024));

    final bytes = Uint8List(bufferLength);
    final stopwatch = Stopwatch()..start();
    final observedLength = await processLargeBytes(bytes: bytes);
    stopwatch.stop();

    if (observedLength != bufferLength) {
      throw StateError(
        'Expected $bufferLength bytes, received $observedLength bytes',
      );
    }

    print(
      'WEB_LARGE_BYTE_TRANSFER bytes=$bufferLength '
      'elapsed_ms=${stopwatch.elapsedMilliseconds}',
    );
  });
}
