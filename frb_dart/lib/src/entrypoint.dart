import 'package:flutter_rust_bridge/src/dispatcher.dart';
import 'package:meta/meta.dart';

abstract class BaseEntrypoint<D extends BaseDispatcher> {
  /// Whether the system has been initialized.
  bool get initialized => _dispatcher != null;

  @internal
  D get dispatcher => _dispatcher ?? (throw StateError('flutter_rust_bridge has not been initialized'));
  D? _dispatcher;

  @protected
  Future<void> initImpl({
    required D dispatcher,
  }) async {
    if (_dispatcher != null) throw StateError('Should not initialize flutter_rust_bridge twice');
    _dispatcher = dispatcher;
    // TODO more init work
  }
}
