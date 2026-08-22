// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/casted_primitive.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(castedPrimitiveI64TwinNormal, <int>[
    0,
    -1000000000,
    1000000000,
  ]);
  addTestsIdentityFunctionCall(castedPrimitiveU64TwinNormal, <int>[
    0,
    1000000000,
  ]);
  addTestsIdentityFunctionCall(castedPrimitiveIsizeTwinNormal, <int>[
    0,
    -1000000000,
    1000000000,
  ]);
  addTestsIdentityFunctionCall(castedPrimitiveUsizeTwinNormal, <int>[
    0,
    1000000000,
  ]);

  test('round trips nested casted primitives', () async {
    final inputs = [
      StructWithCastedPrimitiveTwinNormal(
        fieldI64: -6000000000,
        fieldU64: 7000000000,
        fieldI32: 123456789,
        fieldVecU8: Uint8List.fromList([10, 20, 100]),
        fieldVecI64: [-3000000000, 0, 3000000000],
        fieldOptionalI64: -4000000000,
        fieldBoxedI64: -5000000000,
      ),
      StructWithCastedPrimitiveTwinNormal(
        fieldI64: 6000000000,
        fieldU64: 8000000000,
        fieldI32: -123456789,
        fieldVecU8: Uint8List(0),
        fieldVecI64: const [],
        fieldBoxedI64: 5000000000,
      ),
    ];

    for (final input in inputs) {
      final actual = await functionForStructWithCastedPrimitiveTwinNormal(
        arg: input,
      );

      expect(actual.fieldI64, input.fieldI64);
      expect(actual.fieldU64, input.fieldU64);
      expect(actual.fieldI32, input.fieldI32);
      expect(actual.fieldVecU8, orderedEquals(input.fieldVecU8));
      expect(actual.fieldVecI64, orderedEquals(input.fieldVecI64));
      expect(actual.fieldOptionalI64, input.fieldOptionalI64);
      expect(actual.fieldBoxedI64, input.fieldBoxedI64);
    }
  });
}
