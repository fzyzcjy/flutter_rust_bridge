use flutter_rust_bridge::frb;

#[frb(dart_code = "
  @override
  bool operator ==(StructWithDartCodeTwinNormal other) => a == other.a;

  @override
  int get hashCode => a.hashCode;

  int dartCodeMethod() => a * 2;
")]
pub struct StructWithDartCodeTwinNormal {
    pub a: i32,
}

impl StructWithDartCodeTwinNormal {
    pub fn normal_method(&self) -> i32 {
        self.a * 2
    }
}