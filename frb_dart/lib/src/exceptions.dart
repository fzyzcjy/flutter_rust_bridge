/// Base class for exceptions in flutter_rust_bridge
class FrbException implements Exception {}

/// The rust code is panicked
class PanicException implements FrbException {
  /// The error message
  final String error;

  /// The rust code is panicked
  PanicException(this.error);
}

/// The rust code returns `anyhow::Error`
class FrbAnyhowException implements FrbException {
  /// The error message
  final String anyhow;

  /// The rust code returns `anyhow::Error`
  FrbAnyhowException(this.anyhow);

  @override
  String toString() => 'FrbAnyhowException($anyhow)';
}

/// Interface indicating exceptions that have backtrace (stack trace)
abstract class FrbBacktracedException extends FrbException {
  /// The backtrace (stack trace) of the exception
  String get backtrace;
}

/// Thrown when the browser is not run in a [cross-origin isolated] environment.
///
/// [cross-origin isolated]: https://developer.mozilla.org/en-US/docs/Web/API/crossOriginIsolated
class MissingHeaderException implements FrbException {
  /// Constructs an exception
  const MissingHeaderException();

  static const _message = '''
Buffers cannot be shared due to missing cross-origin headers.
Make sure your web server responds with the following headers:
- Cross-Origin-Opener-Policy: same-origin
- Cross-Origin-Embedder-Policy: credentialless OR require-corp

If running from Flutter, consider `flutter build web` and running a custom static-file server.''';

  @override
  String toString() => _message;
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
