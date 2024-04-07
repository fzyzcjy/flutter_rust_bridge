// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_code.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;

#[frb(dart_code = "
  @override
  bool operator ==(StructWithDartCodeTwinSync other) => a == other.a;

  @override
  int get hashCode => a.hashCode;

  int dartCodeMethod() => a * 2;
")]
pub struct TranslatableStructWithDartCodeTwinSync {
    pub a: i32,
}

impl TranslatableStructWithDartCodeTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn normal_method_twin_sync(&self) -> i32 {
        self.a * 2
    }
}

#[frb(opaque, dart_code = "static int dartCodeGetter => 123;")]
pub struct OpaqueStructWithDartCodeTwinSync;
