// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `collection_equality_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/collection_equality_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('struct_with_deep_collection_equality_uses_content_equality', () {
    final first = StructWithDeepCollectionEqualityTwinSse(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );
    final second = StructWithDeepCollectionEqualityTwinSse(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );

    expect(first, equals(second));
    expect(first.hashCode, second.hashCode);
  });

  test('struct_without_deep_collection_equality_uses_collection_identity', () {
    final first = StructWithShallowCollectionEqualityTwinSse(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );
    final second = StructWithShallowCollectionEqualityTwinSse(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );

    expect(first == second, isFalse);
  });
}
