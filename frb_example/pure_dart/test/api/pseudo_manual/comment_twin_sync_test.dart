// NOTE: This file is auto-generated from `comment_test.dart` by frb_internal
// Please do not modify manually, but modify the `comment_test.dart` and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/comment_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  // do not care too much about the function results, since we are
  // considering the `code comments` feature here, instead of actual function execution logic.
  test('can call the functions', () async {
    functionWithCommentsSlashStarStarTwinSync();
    functionWithCommentsTripleSlashMultiLineTwinSync();
    functionWithCommentsTripleSlashSingleLineTwinSync();
    StructWithCommentsTwinSync(fieldWithComments: 42).instanceMethodTwinSync();
    StructWithCommentsTwinSync.staticMethodTwinSync();
  });
}
