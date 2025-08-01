// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ConstructorOpaqueStructTwinNormal>>
abstract class ConstructorOpaqueStructTwinNormal
    implements RustOpaqueInterface {
  String get one;

  set one(String one);

  void check();

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<ConstructorOpaqueStructTwinNormal> newInstance() =>
      RustLib.instance.api
          .crateApiConstructorConstructorOpaqueStructTwinNormalNew();
}

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<ConstructorOpaqueSyncStructTwinNormal>>
abstract class ConstructorOpaqueSyncStructTwinNormal
    implements RustOpaqueInterface {
  String get one;

  set one(String one);

  void check();

  factory ConstructorOpaqueSyncStructTwinNormal() => RustLib.instance.api
      .crateApiConstructorConstructorOpaqueSyncStructTwinNormalNew();
}

class ConstructorTranslatableStructTwinNormal {
  final String one;

  const ConstructorTranslatableStructTwinNormal({
    required this.one,
  });

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<ConstructorTranslatableStructTwinNormal> newInstance() =>
      RustLib.instance.api
          .crateApiConstructorConstructorTranslatableStructTwinNormalNew();

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConstructorTranslatableStructTwinNormal &&
          runtimeType == other.runtimeType &&
          one == other.one;
}

class ConstructorTranslatableSyncStructTwinNormal {
  final String one;

  const ConstructorTranslatableSyncStructTwinNormal.raw({
    required this.one,
  });

  factory ConstructorTranslatableSyncStructTwinNormal() => RustLib.instance.api
      .crateApiConstructorConstructorTranslatableSyncStructTwinNormalNew();

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is ConstructorTranslatableSyncStructTwinNormal &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
