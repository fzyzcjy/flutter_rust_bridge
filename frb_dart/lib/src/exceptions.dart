class FrbException implements Exception {}

class PanicException implements FrbException {
  final String error;

  PanicException(this.error);
}

class FrbAnyhowException implements FrbException {
  final String anyhow;

  FrbAnyhowException(this.anyhow);

  @override
  String toString() => 'FrbAnyhowException($anyhow)';
}

abstract class FrbBacktracedException extends FrbException {
  String get backtrace;
}

/// Thrown when the browser is not run in a [cross-origin isolated] environment.
///
/// [cross-origin isolated]: https://developer.mozilla.org/en-US/docs/Web/API/crossOriginIsolated
class MissingHeaderException implements FrbException {
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

class PlatformMismatchException implements FrbException {
  const PlatformMismatchException();

  static const _wasm = 'Not implemented on non-WASM platforms';

  @override
  String toString() => _wasm;
}

class UnmodifiableTypedListException implements FrbException {
  const UnmodifiableTypedListException();

  static const _message = 'Cannot modify the length of typed lists.';

  @override
  String toString() => _message;
}
