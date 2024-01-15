// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/comment.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  // do not care too much about the function results, since we are
  // considering the `code comments` feature here, instead of actual function execution logic.
  test('can call the functions', () async {
    await futurizeVoidTwinNormal(functionWithCommentsSlashStarStarTwinNormal());
    await futurizeVoidTwinNormal(
        functionWithCommentsTripleSlashMultiLineTwinNormal());
    await futurizeVoidTwinNormal(
        functionWithCommentsTripleSlashSingleLineTwinNormal());
    await futurizeVoidTwinNormal(
        StructWithCommentsTwinNormal(fieldWithComments: 42)
            .instanceMethodTwinNormal());
    await futurizeVoidTwinNormal(
        StructWithCommentsTwinNormal.staticMethodTwinNormal());
  });
}
