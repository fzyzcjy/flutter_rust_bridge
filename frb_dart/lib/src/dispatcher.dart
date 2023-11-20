import 'package:flutter_rust_bridge/src/handler.dart';

abstract class BaseDispatcher {
  final BaseHandler? handler;

  BaseDispatcher({this.handler});
}
