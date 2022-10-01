import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/interceptor.dart';

class FlutterRustBridgeInterceptor {
  Future<dynamic> beforeExecuteNormal(String benchName) async {
    log('received $benchName');
    return Future.sync(() {
      final TimelineTask task = TimelineTask();
      task.start(benchName);
      return task;
    });
  }

  Future<void> afterExecuteNormal<S>(String benchName, dynamic tracker) async {
    return Future.sync(() {
      final TimelineTask task = tracker as TimelineTask;
      task.finish();
    });
  }

  void beforeExecuteSync<S>(String benchName) {
    log('received $benchName');
    Timeline.startSync(benchName);
  }

  void afterExecuteSync<S>(String benchName, dynamic _) {
    Timeline.finishSync();
  }
}

mixin FlutterRustBridgeInterceptorMixin
    on FlutterRustBridgeExampleBenchmarkSuiteImplBench {
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> op) async {
    log('[[[[[[executeNormal]]]]]]');
    final TimelineTask? task =
        await interceptor.beforeExecuteNormal(op.constMeta.debugName);
    final result = await executeNormal(op);
    if (task != null) {
      await interceptor.afterExecuteNormal(op.constMeta.debugName, task);
    }
    return result;
  }

  S executeSync<S>(FlutterRustBridgeSyncTask op) {
    log('[[[[[[executeSync]]]]]]');
    interceptor.beforeExecuteSync(op.constMeta.debugName);
    final result = executeSync(op);
    interceptor.afterExecuteSync(op.constMeta.debugName, null);
    return result;
  }

  @override
  FlutterRustBridgeInterceptor get interceptor;
}
