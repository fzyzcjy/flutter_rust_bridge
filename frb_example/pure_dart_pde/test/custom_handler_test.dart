// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  test('can use custom handler', () async {
    final customHandler = _MyHandler();
    await RustLib.init(handler: customHandler);
    addTearDown(RustLib.dispose);

    expect(customHandler.logs, <String>[
      'executeNormal init_app',
      'executeNormal my_init_one',
      'executeNormal my_init_two',
      'executeSync frb_internal_logging_allocate_sink_id',
      'executeSync frb_internal_logging_max_level',
      'executeSync frb_internal_init_logger',
      'executeSync frb_internal_logging_setup_dart_logging_output',
      'executeSync record_init_dart_code_message',
      'executeSync record_init_dart_code_message',
    ]);
    expect(await simpleAdderTwinNormal(a: 1, b: 2), 3);
    expect(customHandler.logs, [
      'executeNormal init_app',
      'executeNormal my_init_one',
      'executeNormal my_init_two',
      'executeSync frb_internal_logging_allocate_sink_id',
      'executeSync frb_internal_logging_max_level',
      'executeSync frb_internal_init_logger',
      'executeSync frb_internal_logging_setup_dart_logging_output',
      'executeSync record_init_dart_code_message',
      'executeSync record_init_dart_code_message',
      'executeNormal simple_adder_twin_normal',
    ]);
  });
}

class _MyHandler extends BaseHandler {
  final logs = <String>[];

  @override
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    logs.add('executeNormal ${task.constMeta.debugName}');
    return super.executeNormal(task);
  }

  @override
  S executeSync<S, E extends Object, WireSyncType>(
    SyncTask<S, E, WireSyncType> task,
  ) {
    logs.add('executeSync ${task.constMeta.debugName}');
    return super.executeSync(task);
  }
}
