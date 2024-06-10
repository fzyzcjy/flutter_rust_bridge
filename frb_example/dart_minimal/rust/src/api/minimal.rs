use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

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