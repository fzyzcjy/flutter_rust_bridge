// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored (category: IgnoreBecauseSelfTypeNotAllowed): `method_with_bad_self_twin_sync`

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<MyImplTraitWithSelfTwinSync>>
abstract class MyImplTraitWithSelfTwinSync
    implements RustOpaqueInterface, MyTraitWithSelfTwinSync {
  Future<void> methodWithBadSelfTwinSync(
      {required MyImplTraitWithSelfTwinSync another});

  Future<MyImplTraitWithSelfTwinSync> methodWithGoodSelfTwinSync();
}

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StructOneWithTraitTwinSync>>
abstract class StructOneWithTraitTwinSync
    implements RustOpaqueInterface, SimpleTraitTwinSync {
  int get one;

  set one(int one);

  Future<int> simpleTraitFnReceiverBorrowTwinSync();

  static Future<StructOneWithTraitTwinSync> simpleTraitFnTwinSync(
          {required int value}) =>
      RustLib.instance.api
          .crateApiPseudoManualImplTraitTwinSyncStructOneWithTraitTwinSyncSimpleTraitFnTwinSync(
              value: value);

  static Future<int> simpleTraitFnWithDefaultImplTwinSync() => RustLib
      .instance.api
      .crateApiPseudoManualImplTraitTwinSyncStructOneWithTraitTwinSyncSimpleTraitFnWithDefaultImplTwinSync();
}

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StructTwoWithTraitTwinSync>>
abstract class StructTwoWithTraitTwinSync
    implements RustOpaqueInterface, SimpleTraitTwinSync {
  int get two;

  set two(int two);

  Future<int> simpleTraitFnReceiverBorrowTwinSync();

  static Future<StructTwoWithTraitTwinSync> simpleTraitFnTwinSync(
          {required int value}) =>
      RustLib.instance.api
          .crateApiPseudoManualImplTraitTwinSyncStructTwoWithTraitTwinSyncSimpleTraitFnTwinSync(
              value: value);

  static Future<int> simpleTraitFnWithDefaultImplTwinSync() => RustLib
      .instance.api
      .crateApiPseudoManualImplTraitTwinSyncStructTwoWithTraitTwinSyncSimpleTraitFnWithDefaultImplTwinSync();
}

abstract class MyTraitWithSelfTwinSync {
  Future<MyTraitWithSelfTwinSync> methodWithGoodSelfTwinSync();
}

abstract class SimpleTraitTwinSync {
  Future<int> simpleTraitFnReceiverBorrowTwinSync();
}
