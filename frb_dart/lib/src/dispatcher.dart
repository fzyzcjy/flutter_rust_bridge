import 'package:flutter_rust_bridge/src/handler.dart';
import 'package:meta/meta.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseDispatcher {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  final BaseHandler handler;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseDispatcher({BaseHandler? handler}) : handler = handler ?? BaseHandler();
}
