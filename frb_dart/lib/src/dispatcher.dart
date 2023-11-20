import 'package:flutter_rust_bridge/src/bulk.dart';

/// A thin layer to dispatch calls.
///
/// It exists mainly for testability and separation of concerns.
abstract class BaseDispatcher<B extends BaseBulk> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final B bulk;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const BaseDispatcher({required this.bulk});
}
