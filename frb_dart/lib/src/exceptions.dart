class FrbException implements Exception {}

class PanicException extends FrbException {
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
