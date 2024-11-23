/// Base class for exceptions in flutter_rust_bridge
class FrbException implements Exception {}

/// The rust code is panicked
class PanicException implements FrbException {
  /// The error message
  final String message;

  /// The rust code is panicked
  PanicException(this.message);

  @override
  String toString() => 'PanicException($message)';
}

/// The rust code returns `anyhow::Error`
class AnyhowException implements FrbException {
  /// The error message
  final String message;

  /// The rust code returns `anyhow::Error`
  AnyhowException(this.message);

  @override
  String toString() => 'AnyhowException($message)';
}

/// Interface indicating exceptions that have backtrace (stack trace)
abstract class FrbBacktracedException extends FrbException {
  /// The backtrace (stack trace) of the exception
  String get backtrace;
}

/// Platform is not matched
class PlatformMismatchException implements FrbException {
  /// Constructs an exception
  const PlatformMismatchException();

  static const _wasm = 'Not implemented on non-WASM platforms';

  @override
  String toString() => _wasm;
}

/// Cannot modify a typed list
class UnmodifiableTypedListException implements FrbException {
  /// Constructs an exception
  const UnmodifiableTypedListException();

  static const _message = 'Cannot modify the length of typed lists.';

  @override
  String toString() => _message;
}
