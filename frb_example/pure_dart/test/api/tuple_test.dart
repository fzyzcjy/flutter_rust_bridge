import 'package:frb_example_pure_dart/src/rust/api/tuple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test("dart call tuples", () async {
    expect(testTuple(), completion(('John', 0)));
    expect(testTuple(value: ('Bob', 42)), completion(('Hello Bob', 43)));
  });
}
