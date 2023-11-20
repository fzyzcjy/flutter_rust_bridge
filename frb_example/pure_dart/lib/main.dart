import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:test/test.dart';

void main() {
  test('dart call simpleAdder', () async {
    expect(await simpleAdder(a: 42, b: 100), 142);
  });

  // TODO
}
