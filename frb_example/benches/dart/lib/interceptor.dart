import 'dart:developer';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

class FlutterRustBridgeInterceptor {
  Future<TimelineTask> beforeExecuteNormal(String benchName) async {
    return Future.sync(() {
      final TimelineTask task = TimelineTask();
      task.start(benchName);
      return task;
    });
  }

  Future<void> afterExecuteNormal<S>(
      String benchName, TimelineTask task) async {
    return Future.sync(() {
      task.finish();
    });
  }

  void beforeExecuteSync<S>(
      String benchName, FlutterRustBridgeSyncTask<S> task) {
    Timeline.startSync(benchName);
  }

  void afterExecuteSync<S>(
      String benchName, FlutterRustBridgeSyncTask<S> task) {
    Timeline.finishSync();
  }
}

mixin FlutterRustBridgeInterceptorMixin<T extends FlutterRustBridgeWireBase>
    on FlutterRustBridgeBase<T> {
  @override
  Future<S> executeNormal<S>(FlutterRustBridgeTask<S> scheduledTask) async {
    final TimelineTask? task = await interceptor
        ?.beforeExecuteNormal(scheduledTask.constMeta.debugName);
    final result = await super.executeNormal(scheduledTask);
    if (task != null) {
      await interceptor?.afterExecuteNormal(
          scheduledTask.constMeta.debugName, task);
    }
    return result;
  }

  @override
  S executeSync<S>(FlutterRustBridgeSyncTask task) {
    interceptor?.beforeExecuteSync(task.constMeta.debugName, task);
    final result = super.executeSync(task);
    interceptor?.afterExecuteSync(task.constMeta.debugName, task);
    return result;
  }

  FlutterRustBridgeInterceptor? get interceptor;
}
