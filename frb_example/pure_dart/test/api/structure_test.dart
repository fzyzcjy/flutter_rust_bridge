import 'package:frb_example_pure_dart/src/rust/api/structure.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('call funcStructWithZeroFieldTwinNormal', () async {
    expect(await funcStructWithZeroFieldTwinNormal(arg: const StructWithZeroField()), isA<StructWithZeroField>());
  });
}
