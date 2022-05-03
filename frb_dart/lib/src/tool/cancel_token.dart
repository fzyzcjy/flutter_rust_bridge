import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:meta/meta.dart';

typedef FrbCancelableFutureRunner<R> = Future<R> Function(CancelTokenHandle);

class FrbCancelableFuture<R> {
  final _completer = Completer<R>();
  CancelTokenHandle? _cancelToken;

  FrbCancelableFuture._(this._cancelToken);

  static Future<FrbCancelableFuture<R>> create<R>({
    required int subPoolId,
    required FrbCancelableFutureRunner<R> runner,
  }) async {
    final ans = FrbCancelableFuture<R>._(await nativeVisionUtils.cancelTokenNew(subPoolId: subPoolId));
    unawaited(ans._run(runner));
    return ans;
  }

  Future<void> _run(FrbCancelableFutureRunner<R> runner) async {
    final outerStackTrace = StackTrace.current;
    try {
      final result = await runner(_cancelToken!);
      _completer.complete(result);
    } catch (e, s) {
      // NOTE if directly [completer.completeError], will lose outer stack trace, thus have to bind #1762
      ErrorHandlerService.I.bindExtraToException(e, 'outerStacktrace=$outerStackTrace');
      _completer.completeError(e, s);
    } finally {
      await _removeCancelToken(); // 别忘了清理资源: remove它
    }
  }

  Future<void> _removeCancelToken() async {
    final cancelTokenToRemove = _cancelToken!;

    _cancelToken = null; // set to null to avoid misuse

    await nativeVisionUtils.cancelTokenRemove(cancelToken: cancelTokenToRemove);
  }

  Future<bool> maybeCancel() async {
    final cancelToken = _cancelToken;
    if (_completer.isCompleted || cancelToken == null) return false;

    await nativeVisionUtils.cancelTokenCancel(cancelToken: cancelToken);
    return true;
  }

  bool get isCompleted => _completer.isCompleted;

  Future<R> get future => _completer.future;
}
