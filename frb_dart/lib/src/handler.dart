class BaseHandler {
  Future<S> executeNormal<S, E extends Object>(FlutterRustBridgeTask<S, E> task) {
    return TODO;
  }

  S executeSync<S, E extends Object>(FlutterRustBridgeSyncTask<S, E> task) {
    return TODO;
  }

  Stream<S> executeStream<S, E extends Object>(FlutterRustBridgeTask<S, E> task) {
    return TODO;
  }
}
