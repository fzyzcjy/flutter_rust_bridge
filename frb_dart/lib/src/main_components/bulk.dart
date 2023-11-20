import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:meta/meta.dart';

/// The main workhorse for Dart-Rust calls, thus it is called "bulk".
abstract class BaseBulk {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @protected
  final BaseHandler handler;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseBulk({BaseHandler? handler}) : handler = handler ?? BaseHandler();
}
