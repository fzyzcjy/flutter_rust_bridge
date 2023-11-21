import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/main_components/api.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:meta/meta.dart';

/// This is the main entrypoint.
/// For example, users call `init` on it, and auto-generated code call `api` on it.
///
/// This class is like "service locator" (e.g. the get_it package) for all services related to flutter_rust_bridge.
///
/// This should be a singleton per flutter_rust_bridge usage (enforced via generated subclass code).
abstract class BaseEntrypoint<A extends BaseApi> {
  /// Whether the system has been initialized.
  bool get initialized => __state != null;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  A get api => _state.api;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @internal
  NativePortType get dropPort => _state.dropPortManager.dropPort;

  _EntrypointState<A> get _state => __state ?? (throw StateError('flutter_rust_bridge has not been initialized'));
  _EntrypointState<A>? __state;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  Future<void> initImpl({
    A? api,
    BaseHandler? handler,
  }) async {
    if (__state != null) throw StateError('Should not initialize flutter_rust_bridge twice');
    __state = _EntrypointState(api: api ?? createDefaultDispatcher(handler: handler));
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  void disposeImpl() {
    __state!.dispose();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  A createDefaultDispatcher({BaseHandler? handler});
}

class _EntrypointState<A extends BaseApi> {
  final A api;
  late final dropPortManager = _DropPortManager(TODO);

  _EntrypointState({required this.api}) {
    _setUpRustToDartCommunication(TODO);
  }

  void dispose() {
    dropPortManager.dispose();
  }
}

class _DropPortManager {
  final GeneralizedFrbRustBinding _generalizedFrbRustBinding;

  _DropPortManager(this._generalizedFrbRustBinding);

  NativePortType get dropPort => _dropPort.sendPort.nativePort;
  late final _dropPort = _initDropPort();

  ReceivePort _initDropPort() {
    final port = broadcastPort(DropIdPortGenerator.create());
    port.listen((message) {
      _generalizedFrbRustBinding.dropDartObject(message);
    });
    return port;
  }

  void dispose() {
    _dropPort.close();
  }
}

void _setUpRustToDartCommunication(GeneralizedFrbRustBinding generalizedFrbRustBinding) {
  generalizedFrbRustBinding.storeDartPostCObject();
}
