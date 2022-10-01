import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

class FlutterRustBridgeInterceptor {
  Future<dynamic> beforeExecuteNormal(String benchName) async {
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
    Timeline.startSync(benchName);
  }

  void afterExecuteSync<S>(String benchName, dynamic _) {
    Timeline.finishSync();
  }
}

mixin FlutterRustBridgeInterceptorMixin<T extends FlutterRustBridgeWireBase>
    on FlutterRustBridgeBase<T> {
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> op) async {
    final TimelineTask? task =
        await interceptor.beforeExecuteNormal(op.constMeta.debugName);
    final result = await super.executeNormal(op);
    if (task != null) {
      await interceptor.afterExecuteNormal(op.constMeta.debugName, task);
    }
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask op) {
    interceptor.beforeExecuteSync(op.constMeta.debugName);
    final result = super.executeSync(op);
    interceptor.afterExecuteSync(op.constMeta.debugName, null);
    return result;
  }

  FlutterRustBridgeInterceptor get interceptor;
}
