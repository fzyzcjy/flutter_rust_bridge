// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

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

#[frb(opaque, dart_code = "static int get dartCodeGetter => 123;")]
pub struct OpaqueStructWithDartCodeTwinNormal;

impl OpaqueStructWithDartCodeTwinNormal {
    pub fn normal_method_twin_normal(&self) -> i32 {
        100
    }
}
