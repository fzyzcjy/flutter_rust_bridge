// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.37.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StructOneWithTraitTwinRustAsync>>
abstract class StructOneWithTraitTwinRustAsync
    implements SimpleTraitTwinRustAsync {
  int get one;

  void set one(int one);

  Future<int> simpleTraitFnReceiverBorrowTwinRustAsync();

  static Future<StructOneWithTraitTwinRustAsync> simpleTraitFnTwinRustAsync(
          {required int value}) =>
      RustLib.instance.api
          .crateApiPseudoManualImplTraitTwinRustAsyncStructOneWithTraitTwinRustAsyncSimpleTraitFnTwinRustAsync(
              value: value);

  static Future<int> simpleTraitFnWithDefaultImplTwinRustAsync() => RustLib
      .instance.api
      .crateApiPseudoManualImplTraitTwinRustAsyncStructOneWithTraitTwinRustAsyncSimpleTraitFnWithDefaultImplTwinRustAsync();

  void dispose();

  bool get isDisposed;
}

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<StructTwoWithTraitTwinRustAsync>>
abstract class StructTwoWithTraitTwinRustAsync
    implements SimpleTraitTwinRustAsync {
  int get two;

  void set two(int two);

  Future<int> simpleTraitFnReceiverBorrowTwinRustAsync();

  static Future<StructTwoWithTraitTwinRustAsync> simpleTraitFnTwinRustAsync(
          {required int value}) =>
      RustLib.instance.api
          .crateApiPseudoManualImplTraitTwinRustAsyncStructTwoWithTraitTwinRustAsyncSimpleTraitFnTwinRustAsync(
              value: value);

  static Future<int> simpleTraitFnWithDefaultImplTwinRustAsync() => RustLib
      .instance.api
      .crateApiPseudoManualImplTraitTwinRustAsyncStructTwoWithTraitTwinRustAsyncSimpleTraitFnWithDefaultImplTwinRustAsync();

  void dispose();

  bool get isDisposed;
}

abstract class SimpleTraitTwinRustAsync {
  Future<int> simpleTraitFnReceiverBorrowTwinRustAsync();
}