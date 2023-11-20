import 'package:flutter_rust_bridge/src/dispatcher.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:meta/meta.dart';

import 'generalized_isolate/generalized_isolate.dart';

abstract class BaseEntrypoint<D extends BaseDispatcher> {
  /// Whether the system has been initialized.
  bool get initialized => __state != null;

  @internal
  D get dispatcher => _state.dispatcher;

  @internal
  NativePortType get dropPort => _state.dropPortManager.dropPort;

  _EntrypointState<D> get _state => __state ?? (throw StateError('flutter_rust_bridge has not been initialized'));
  _EntrypointState<D>? __state;

  @protected
  Future<void> initImpl({
    D? dispatcher,
  }) async {
    if (initialized) throw StateError('Should not initialize flutter_rust_bridge twice');
    __state = _EntrypointState(dispatcher: dispatcher ?? createDefaultDispatcher());
  }

  @protected
  void disposeImpl() {
    __state!.dispose();
    __state = null;
  }

  @protected
  D createDefaultDispatcher();
}

class _EntrypointState<D extends BaseDispatcher> {
  final D dispatcher;
  final dropPortManager = _DropPortManager();

  _EntrypointState({required this.dispatcher}) {
    _setUpRustToDartCommunication();
  }

  void dispose() {
    dropPortManager.dispose();
  }
}

class _DropPortManager {
  NativePortType get dropPort => _dropPort.sendPort.nativePort;
  late final _dropPort = _initDropPort();

  ReceivePort _initDropPort() {
    final port = broadcastPort(DropIdPortGenerator.create());
    port.listen((message) {
      inner.drop_dart_object(message);
    });
    return port;
  }

  void dispose() {
    _dropPort.close();
  }
}

void _setUpRustToDartCommunication() {
  inner.storeDartPostCObject();
}
