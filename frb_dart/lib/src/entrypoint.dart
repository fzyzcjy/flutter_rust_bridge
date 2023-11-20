import 'package:flutter_rust_bridge/src/dispatcher.dart';
import 'package:meta/meta.dart';

abstract class BaseEntrypoint<D extends BaseDispatcher> {
  /// Whether the system has been initialized.
  bool get initialized => __state != null;

  @internal
  D get dispatcher => _state.dispatcher;

  _EntrypointState<D> get _state => __state ?? (throw StateError('flutter_rust_bridge has not been initialized'));
  _EntrypointState<D>? __state;

  @protected
  Future<void> initImpl({
    D? dispatcher,
  }) async {
    if (initialized) throw StateError('Should not initialize flutter_rust_bridge twice');
    __state = _EntrypointState(dispatcher: dispatcher ?? createDefaultDispatcher());
    _sanityCheckSingleton(runtimeType);
  }

  @protected
  D createDefaultDispatcher();
}

class _EntrypointState<D extends BaseDispatcher> {
  final D dispatcher;

  const _EntrypointState({required this.dispatcher});
}

void _sanityCheckSingleton(Type entrypointRuntimeType) {
  if (_instances.contains(entrypointRuntimeType)) {
    throw Exception(
      'Subclasses of `BaseEntrypoint` should be singletons - '
      'there should not be two instances (runtimeType=$entrypointRuntimeType)',
    );
  }
  _instances.add(runtimeType);
}
