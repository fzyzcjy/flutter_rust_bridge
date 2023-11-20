import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/main_components/dispatcher.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:meta/meta.dart';

/// This is the main entrypoint.
/// For example, users call `init` on it, and auto-generated code call `dispatcher` on it.
///
/// This class is like "service locator" (e.g. the get_it package) for all services related to flutter_rust_bridge.
///
/// This should be a singleton per flutter_rust_bridge usage (enforced via generated subclass code).
abstract class BaseEntrypoint<D extends BaseDispatcher> {
  /// Whether the system has been initialized.
  bool get initialized => __state != null;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  D get dispatcher => _state.dispatcher;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  NativePortType get dropPort => _state.dropPortManager.dropPort;

  _EntrypointState<D> get _state => __state ?? (throw StateError('flutter_rust_bridge has not been initialized'));
  _EntrypointState<D>? __state;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  Future<void> initImpl({
    D? dispatcher,
    BaseHandler? handler,
  }) async {
    if (__state != null) throw StateError('Should not initialize flutter_rust_bridge twice');
    __state = _EntrypointState(dispatcher: dispatcher ?? createDefaultDispatcher(handler: handler));
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  void disposeImpl() {
    __state!.dispose();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  D createDefaultDispatcher({BaseHandler? handler});
}

class _EntrypointState<D extends BaseDispatcher> {
  final D dispatcher;
  late final dropPortManager = _DropPortManager(dispatcher);

  _EntrypointState({required this.dispatcher}) {
    _setUpRustToDartCommunication(dispatcher);
  }

  void dispose() {
    dropPortManager.dispose();
  }
}

class _DropPortManager {
  final BaseDispatcher _dispatcher;

  _DropPortManager(this._dispatcher);

  NativePortType get dropPort => _dropPort.sendPort.nativePort;
  late final _dropPort = _initDropPort();

  ReceivePort _initDropPort() {
    final port = broadcastPort(DropIdPortGenerator.create());
    port.listen((message) {
      _dispatcher.inner.drop_dart_object(message);
    });
    return port;
  }

  void dispose() {
    _dropPort.close();
  }
}

void _setUpRustToDartCommunication(BaseDispatcher dispatcher) {
  dispatcher.inner.storeDartPostCObject();
}
