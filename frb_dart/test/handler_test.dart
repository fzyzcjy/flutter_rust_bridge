import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/api_impl.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/task.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

class _MockApiImpl extends Mock implements BaseApiImpl {}

class _MockBinding extends Mock implements GeneralizedFrbRustBinding {}

class _TrackingCodec extends BaseCodec<int, FrbException, int> {
  int? freedValue;
  GeneralizedFrbRustBinding? freedBinding;

  @override
  int decodeObject(dynamic raw) => raw as int;

  @override
  int decodeWireSyncType(int raw) => raw * 2;

  @override
  void freeWireSyncRust2Dart(
    int raw,
    GeneralizedFrbRustBinding generalizedFrbRustBinding,
  ) {
    freedValue = raw;
    freedBinding = generalizedFrbRustBinding;
  }
}

void main() {
  test('executeSync decodes and always frees the native return value', () {
    final binding = _MockBinding();
    final apiImpl = _MockApiImpl();
    final codec = _TrackingCodec();
    when(() => apiImpl.generalizedFrbRustBinding).thenReturn(binding);
    final task = SyncTask<int, FrbException, int>(
      callFfi: () => 21,
      codec: codec,
      constMeta: const TaskConstMeta(debugName: 'sync', argNames: []),
      argValues: const [],
      apiImpl: apiImpl,
    );

    expect(BaseHandler().executeSync(task), 42);
    expect(codec.freedValue, 21);
    expect(codec.freedBinding, same(binding));
  });

  test('executeSync preserves FRB exceptions without translating them', () {
    final expected = PanicException('rust panic');
    final task = SyncTask<int, FrbException, int>(
      callFfi: () => throw expected,
      codec: _TrackingCodec(),
      constMeta: const TaskConstMeta(debugName: 'sync', argNames: []),
      argValues: const [],
      apiImpl: _MockApiImpl(),
    );

    expect(() => BaseHandler().executeSync(task), throwsA(same(expected)));
  });

  test('dartFnInvoke decodes a closure and forwards every argument', () {
    final binding = _MockBinding();
    Object? receivedFirst;
    Object? receivedSecond;
    when(() => binding.dartOpaqueRust2DartDecode(7)).thenReturn((a, b) {
      receivedFirst = a;
      receivedSecond = b;
    });

    BaseHandler().dartFnInvoke([7, 'first', 2], binding);

    expect(receivedFirst, 'first');
    expect(receivedSecond, 2);
  });
}
