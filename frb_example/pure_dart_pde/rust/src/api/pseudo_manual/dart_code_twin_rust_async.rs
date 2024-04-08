// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_code.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;

#[frb(
    non_hash,
    non_eq,
    dart_code = "
  @override
  bool operator ==(Object other) =>
    other is TranslatableStructWithDartCodeTwinRustAsync && a == other.a;

  @override
  int get hashCode => a.hashCode;

  int dartCodeMethod() => a * 2;
"
)]
pub struct TranslatableStructWithDartCodeTwinRustAsync {
    pub a: i32,
}

impl TranslatableStructWithDartCodeTwinRustAsync {
    pub async fn normal_method_twin_rust_async(&self) -> i32 {
        self.a * 2
    }
}

#[frb(opaque, dart_code = "static int dartCodeGetter => 123;")]
pub struct OpaqueStructWithDartCodeTwinRustAsync;

impl OpaqueStructWithDartCodeTwinRustAsync {
    pub async fn normal_method_twin_rust_async(&self) -> i32 {
        100
    }
}
