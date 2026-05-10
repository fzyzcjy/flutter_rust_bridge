// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `collection_equality_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/collection_equality_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('struct_with_deep_collection_equality_uses_content_equality', () {
    final first = _makeDeepCollectionStruct();
    final second = _makeDeepCollectionStruct();

    expect(first, equals(second));
    expect(first.hashCode, second.hashCode);
  });

  test('struct_with_deep_collection_equality_compares_each_collection_field',
      () {
    final original = _makeDeepCollectionStruct();
    final variants = [
      _makeDeepCollectionStruct(list: ['changed']),
      _makeDeepCollectionStruct(map: {'k': 'changed'}),
      _makeDeepCollectionStruct(setValues: {'changed'}),
      _makeDeepCollectionStruct(optionalList: ['changed']),
      _makeDeepCollectionStruct(bytes: Uint8List.fromList([9, 2, 3])),
    ];

    for (final variant in variants) {
      expect(original == variant, isFalse);
    }
  });

  test('struct_without_deep_collection_equality_uses_collection_identity', () {
    final first = _makeShallowCollectionStruct();
    final second = _makeShallowCollectionStruct();

    expect(first == second, isFalse);
  });
}

StructWithDeepCollectionEqualityTwinSync _makeDeepCollectionStruct({
  List<String>? list,
  Map<String, String>? map,
  Set<String>? setValues,
  List<String>? optionalList,
  Uint8List? bytes,
}) =>
    StructWithDeepCollectionEqualityTwinSync(
      list: list ?? ['a'],
      map: map ?? {'k': 'v'},
      setValues: setValues ?? {'x'},
      optionalList: optionalList ?? ['optional'],
      bytes: bytes ?? Uint8List.fromList([1, 2, 3]),
    );

StructWithShallowCollectionEqualityTwinSync _makeShallowCollectionStruct() =>
    StructWithShallowCollectionEqualityTwinSync(
      list: ['a'],
      map: {'k': 'v'},
      setValues: {'x'},
      optionalList: ['optional'],
      bytes: Uint8List.fromList([1, 2, 3]),
    );
