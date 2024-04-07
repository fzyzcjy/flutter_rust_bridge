use flutter_rust_bridge::frb;

#[frb(
    non_hash,
    non_eq,
    dart_code = "
  @override
  bool operator ==(StructWithDartCodeTwinNormal other) => a == other.a;

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

#[frb(opaque, dart_code = "static int dartCodeGetter => 123;")]
pub struct OpaqueStructWithDartCodeTwinNormal;

impl OpaqueStructWithDartCodeTwinNormal {
    pub fn normal_method_twin_normal(&self) -> i32 {
        self.a * 2
    }
}
