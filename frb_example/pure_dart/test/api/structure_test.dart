import 'package:frb_example_pure_dart/src/rust/api/structure.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcStructWithZeroFieldTwinNormal', () async {
    expect(await funcStructWithZeroFieldTwinNormal(arg: const StructWithZeroField()), isA<StructWithZeroField>());
  });

  test('call funcStructWithOneFieldTwinNormal', () async {
    expect(await funcStructWithOneFieldTwinNormal(arg: const StructWithOneField(a: 42)),
        isA<StructWithOneField>().having((x) => x.a, 'a', 42));
  });

  test('call funcStructWithTwoFieldTwinNormal', () async {
    expect(await funcStructWithTwoFieldTwinNormal(arg: const StructWithTwoField(a: 10, b: 20)),
        isA<StructWithTwoField>().having((x) => x.a, 'a', 10).having((x) => x.b, 'b', 20));
  });

  test('call funcTupleStructWithOneFieldTwinNormal', () async {
    expect(await funcTupleStructWithOneFieldTwinNormal(arg: const TupleStructWithOneField(field0: 10)),
        isA<TupleStructWithOneField>().having((x) => x.field0, 'field0', 10));
  });

  test('call funcTupleStructWithTwoFieldTwinNormal', () async {
    expect(await funcTupleStructWithTwoFieldTwinNormal(arg: const TupleStructWithTwoField(field0: 10, field1: 20)),
        isA<TupleStructWithTwoField>().having((x) => x.field0, 'field0', 10).having((x) => x.field1, 'field1', 20));
  });
}
