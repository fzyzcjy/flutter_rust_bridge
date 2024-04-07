// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_code.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;

#[frb(
    non_hash,
    non_eq,
    dart_code = "
  @override
  bool operator ==(StructWithDartCodeTwinSse other) => a == other.a;

  @override
  int get hashCode => a.hashCode;

  int dartCodeMethod() => a * 2;
"
)]
pub struct TranslatableStructWithDartCodeTwinSse {
    pub a: i32,
}

impl TranslatableStructWithDartCodeTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn normal_method_twin_sse(&self) -> i32 {
        self.a * 2
    }
}

#[frb(opaque, dart_code = "static int dartCodeGetter => 123;")]
pub struct OpaqueStructWithDartCodeTwinSse;

impl OpaqueStructWithDartCodeTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn normal_method_twin_sse(&self) -> i32 {
        self.a * 2
    }
}
