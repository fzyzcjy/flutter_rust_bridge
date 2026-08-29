@TestOn('vm')
import 'dart:async';

import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/main_components/api_impl.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/task.dart';
import 'package:flutter_rust_bridge/src/utils/timeout_handler_mixin.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

class _MockApiImpl extends Mock implements BaseApiImpl {}

class _MockCodec extends Mock
    implements BaseCodec<int, FrbException, dynamic> {}

class _TimeoutHandler extends BaseHandler with TimeoutHandlerMixin {
  @override
  final Duration? timeLimitForExecuteNormal;

  _TimeoutHandler(this.timeLimitForExecuteNormal);
}

NormalTask<int, FrbException> _task({
  required void Function(int port) callFfi,
  String debugName = 'test_task',
}) => NormalTask(
  callFfi: callFfi,
  codec: _MockCodec(),
  constMeta: TaskConstMeta(debugName: debugName, argNames: const []),
  argValues: const [],
  apiImpl: _MockApiImpl(),
);

void main() {
  test(
    'executeNormal completes with timeout details after its configured limit',
    () async {
      const duration = Duration.zero;
      final future = _TimeoutHandler(
        duration,
      ).executeNormal(_task(callFfi: (_) {}));

      await expectLater(
        future,
        throwsA(
          isA<FrbTimeoutException>()
              .having((exception) => exception.duration, 'duration', duration)
              .having(
                (exception) => exception.debugName,
                'debugName',
                'test_task',
              )
              .having(
                (exception) => exception.stackTrace,
                'stackTrace',
                isNotNull,
              ),
        ),
      );
    },
  );

  test(
    'executeNormal leaves a pending task pending when timeout is disabled',
    () async {
      var didCallFfi = false;
      var didComplete = false;
      final future = _TimeoutHandler(
        null,
      ).executeNormal(_task(callFfi: (_) => didCallFfi = true));
      unawaited(
        future.then((_) {
          didComplete = true;
        }),
      );

      await Future<void>.delayed(Duration.zero);

      expect(didCallFfi, isTrue);
      expect(didComplete, isFalse);
    },
  );

  test('executeNormal preserves synchronous source failures', () {
    final expected = StateError('ffi failed');

    expect(
      () => _TimeoutHandler(
        const Duration(days: 1),
      ).executeNormal(_task(callFfi: (_) => throw expected)),
      throwsA(same(expected)),
    );
  });
}
