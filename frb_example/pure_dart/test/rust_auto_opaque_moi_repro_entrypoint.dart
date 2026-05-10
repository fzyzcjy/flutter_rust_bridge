import 'dart:async';

import 'package:flutter_rust_bridge_utils/flutter_rust_bridge_utils_web.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await dartWebTestEntrypoint(() async {
    await RustLib.init();

    test('dispose while workers mutate MoiArc pool does not block main thread',
        () async {
      for (var iteration = 0; iteration < 50; ++iteration) {
        final objects = await Future.wait([
          for (var index = 0; index < 32; ++index)
            rustAutoOpaqueReturnOwnTwinMoi(initial: iteration * 1000 + index),
        ]);

        final futures = [
          for (var index = 0; index < 128; ++index)
            rustAutoOpaqueReturnOwnTwinMoi(initial: iteration * 10000 + index),
        ];

        for (final object in objects) {
          object.dispose();
        }

        final created = await Future.wait(futures);
        for (final object in created) {
          object.dispose();
        }

        await Future<void>.delayed(Duration.zero);
      }
    });
  });
}
