import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

class _MockBaseApiImpl extends Mock implements BaseApiImpl {}

class _MockBaseCodec<S, E extends Object, WireSyncType> extends Mock
    implements BaseCodec<S, E, WireSyncType> {}

void main() {
  test('task-related', () {
    const meta = TaskConstMeta(
      debugName: 'my_name',
      argNames: ['k'],
    );
    const metaTwo = TaskConstMeta(
      debugName: 'my_name',
      argNames: ['k'],
    );
    final task = NormalTask(
      callFfi: (_) {},
      codec: _MockBaseCodec(),
      constMeta: meta,
      argValues: const ['v'],
      apiImpl: _MockBaseApiImpl(),
      hint: null,
    );

    expect(task.argMap, {'k': 'v'});
    expect(meta == metaTwo, true);
    expect(meta.hashCode == metaTwo.hashCode, true);
    expect(meta.toString(), contains('TaskConstMeta'));
  });
}
