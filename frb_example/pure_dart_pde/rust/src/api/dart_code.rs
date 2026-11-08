// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

#![allow(clippy::new_without_default)]

use flutter_rust_bridge::frb;

#[frb(
    non_hash,
    non_eq,
    dart_code = "
  import 'dart:math';

  int dummyMethodThatNeedsDartImport() => Random().nextInt(10);

  @override
  bool operator ==(Object other) =>
    other is TranslatableStructWithDartCodeTwinNormal && a == other.a;

  @override
  int get hashCode => a.hashCode;

  int dartCodeMethod() => a * 2;
"
)]
pub struct TranslatableStructWithDartCodeTwinNormal {
    pub a: i32,
}

impl TranslatableStructWithDartCodeTwinNormal {
    pub fn normal_method_twin_normal(&self) -> i32 {
        self.a * 2
    }
}

#[frb(
    opaque,
    dart_code_mixin,
    dart_code = "
  static int get dartCodeGetter => 123;

  Future<int> dartCodeMethod() async => await normalMethodTwinNormal() * 2;

  @override
  bool operator ==(Object other) => other is OpaqueStructWithDartCodeTwinNormal;

  @override
  int get hashCode => 7;

  @override
  String toString() => 'OpaqueStructWithDartCodeTwinNormal(dart_code)';
"
)]
pub struct OpaqueStructWithDartCodeTwinNormal;

impl OpaqueStructWithDartCodeTwinNormal {
    #[frb(sync)]
    pub fn new() -> Self {
        Self
    }

    pub fn normal_method_twin_normal(&self) -> i32 {
        100
    }
}

#[frb(
    opaque,
    dart_code_mixin,
    dart_code = "
  Future<int> dartCodeMethod() async => await normalMethodTwinNormal() * 2;

  @override
  String toString() => 'ProxiedOpaqueStructWithDartCodeTwinNormal(dart_code)';
"
)]
pub struct ProxiedOpaqueStructWithDartCodeTwinNormal;

impl ProxiedOpaqueStructWithDartCodeTwinNormal {
    #[frb(serialize)]
    pub fn normal_method_twin_normal(&self) -> i32 {
        100
    }
}

#[frb(opaque)]
pub struct ProxiedOpaqueStructWithDartCodeOwnerTwinNormal {
    inner: ProxiedOpaqueStructWithDartCodeTwinNormal,
}

impl ProxiedOpaqueStructWithDartCodeOwnerTwinNormal {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            inner: ProxiedOpaqueStructWithDartCodeTwinNormal,
        }
    }

    #[frb(proxy)]
    #[frb(serialize)]
    pub fn inner_twin_normal(&self) -> &ProxiedOpaqueStructWithDartCodeTwinNormal {
        &self.inner
    }
}
