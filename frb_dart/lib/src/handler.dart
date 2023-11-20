import 'package:flutter_rust_bridge/src/task.dart';

class BaseHandler {
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    return TODO;
  }

  S executeSync<S, E extends Object>(SyncTask<S, E> task) {
    return TODO;
  }

  Stream<S> executeStream<S, E extends Object>(StreamTask<S, E> task) {
    return TODO;
  }
}
