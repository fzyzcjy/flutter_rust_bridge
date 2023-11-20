import 'package:flutter_rust_bridge/src/handler.dart';
import 'package:meta/meta.dart';

abstract class BaseDispatcher {
  @protected
  final BaseHandler handler;

  BaseDispatcher({BaseHandler? handler}) : handler = handler ?? BaseHandler();
}
