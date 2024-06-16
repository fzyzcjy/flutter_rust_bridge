use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// ----------------------------------------------- tests -------------------------------------------------

/// Lt := Lifetime Testers
/// Try *NOT* to impl Clone for these types in order to ensure there are no extra clones
#[frb(opaque)]
#[derive(Debug)]
pub struct LtOwnedStructTwinNormal {
    sub: LtOwnedSubStructTwinNormal,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtOwnedSubStructTwinNormal {
    value: String,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtTypeWithLifetimeTwinNormal<'a> {
    foo: &'a LtOwnedStructTwinNormal,
}

#[allow(clippy::needless_lifetimes)]
impl LtOwnedStructTwinNormal {
    pub fn compute_type_with_lifetime_twin_normal<'a>(
        &'a self,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        LtTypeWithLifetimeTwinNormal { foo: self }
    }

    pub fn compute_sub_struct_twin_normal<'a>(&'a self) -> &'a LtOwnedSubStructTwinNormal {
        &self.sub
    }
}
