import 'package:frb_example_pure_dart/src/rust/api/collection_equality.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('struct_with_deep_collection_equality_uses_content_equality', () {
    final first = StructWithDeepCollectionEqualityTwinNormal(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );
    final second = StructWithDeepCollectionEqualityTwinNormal(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );

    expect(first, equals(second));
    expect(first.hashCode, second.hashCode);
  });

  test('struct_without_deep_collection_equality_uses_collection_identity', () {
    final first = StructWithShallowCollectionEqualityTwinNormal(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );
    final second = StructWithShallowCollectionEqualityTwinNormal(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
    );

    expect(first == second, isFalse);
  });
}
