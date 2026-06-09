@TestOn('vm')
import 'package:flutter_rust_bridge/src/dart_opaque/_common.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/main_components/handler.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:test/test.dart';

void main() {
  test('Dart opaque decoder accepts integral JavaScript numbers', () {
    final binding = _FakeGeneralizedFrbRustBinding(
      values: {7: 'decoded opaque'},
    );

    expect(decodeDartOpaqueCommon(7.0, binding), 'decoded opaque');
    expect(binding.lastDecodedPointer, 7);
  });

  test('Dart opaque decoder reports missing fake pointers in tests', () {
    final binding = _FakeGeneralizedFrbRustBinding(values: const {});

    expect(
      () => decodeDartOpaqueCommon(7.0, binding),
      throwsA(
        isA<StateError>().having(
          (error) => error.message,
          'message',
          'Missing fake Dart opaque value for pointer 7',
        ),
      ),
    );
  });

  test('Dart function handler accepts integral JavaScript call ids', () {
    var observedCallId = -1;
    var observedArgument = '';
    final binding = _FakeGeneralizedFrbRustBinding(
      values: {
        9: (int callId, String argument) {
          observedCallId = callId;
          observedArgument = argument;
        },
      },
    );

    BaseHandler().dartFnInvoke([9, 42.0, 'payload'], binding);

    expect(observedCallId, 42);
    expect(observedArgument, 'payload');
  });

  test('Dart function handler rejects non-integral JavaScript call ids', () {
    final binding = _FakeGeneralizedFrbRustBinding(values: {9: (int _) {}});

    expect(
      () => BaseHandler().dartFnInvoke([9, 42.5], binding),
      throwsA(
        isA<Exception>().having(
          (error) => error.toString(),
          'message',
          contains('decodeProtocolInt see unexpected type=double value=42.5'),
        ),
      ),
    );
  });

  test('DCO string decoder accepts integer character codes', () {
    expect(dcoDecodeString(65), 'A');
  });

  test('Fake binding encodes Dart opaque values as null pointers', () {
    final binding = _FakeGeneralizedFrbRustBinding(values: const {});

    final pointer = binding.dartOpaqueDart2RustEncode(Object(), 0);

    expect(PlatformPointerUtil.isNullPtr(pointer), isTrue);
  });
}

class _FakeGeneralizedFrbRustBinding extends GeneralizedFrbRustBinding {
  final Map<int, Object> values;
  int? lastDecodedPointer;

  _FakeGeneralizedFrbRustBinding({required this.values})
    : super(ExternalLibrary.process(iKnowHowToUseIt: true));

  @override
  Object dartOpaqueRust2DartDecode(int ptr) {
    lastDecodedPointer = ptr;
    final value = values[ptr];
    if (value == null) {
      throw StateError('Missing fake Dart opaque value for pointer $ptr');
    }
    return value;
  }

  @override
  PlatformPointer dartOpaqueDart2RustEncode(
    Object object,
    NativePortType dartHandlerPort,
  ) => PlatformPointerUtil.nullPtr();
}
