/// {@macro flutter_rust_bridge.only_for_generated_code}
mixin SimpleDisposable {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dispose() {
    _isDisposed = true;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  bool get isDisposed => _isDisposed;
  bool _isDisposed = false;
}
