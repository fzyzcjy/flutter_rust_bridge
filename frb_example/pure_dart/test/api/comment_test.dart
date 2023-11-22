import 'package:frb_example_pure_dart/src/rust/api/comment.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  // do not care too much about the function results, since we are
  // considering the `code comments` feature here, instead of actual function execution logic.
  test('can call the functions', () async {
    functionWithCommentsSlashStarStar();
    functionWithCommentsTripleSlashMultiLine();
    functionWithCommentsTripleSlashSingleLine();
    StructWithComments(fieldWithComments: 42).instanceMethod();
    StructWithComments.staticMethod();
  });
}
