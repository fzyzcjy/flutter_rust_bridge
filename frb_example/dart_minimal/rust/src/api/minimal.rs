#![allow(clippy::needless_lifetimes)]

use flutter_rust_bridge::for_generated::lazy_static;
use flutter_rust_bridge::frb;
use std::sync::Mutex;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// ----------------------------------------------- tests -------------------------------------------------
pub(crate) struct SimpleLogger(Mutex<Vec<String>>);

impl SimpleLogger {
    pub(crate) fn new() -> Self {
        Self(Mutex::new(vec![]))
    }
    pub(crate) fn log(&self, message: String) {
        self.0.lock().unwrap().push(message);
    }

    pub(crate) fn get_and_reset(&self) -> Vec<String> {
        self.0.lock().unwrap().drain(..).collect()
    }
}

// -------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LOGGER: SimpleLogger = SimpleLogger::new();
}

/// Lt := Lifetime Testers
pub fn lt_get_and_reset_logs_twin_normal() -> Vec<String> {
    LOGGER.get_and_reset()
}

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
    field: &'a LtOwnedStructTwinNormal,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtNestedTypeWithLifetimeTwinNormal<'a> {
    field: &'a LtTypeWithLifetimeTwinNormal<'a>,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtTypeWithMultiOwnerTwinNormal<'a> {
    fields: Vec<&'a LtOwnedStructTwinNormal>,
}

impl LtOwnedStructTwinNormal {
    pub fn create(value: String) -> Self {
        Self {
            sub: LtOwnedSubStructTwinNormal { value },
        }
    }

    /// `fn f(x: &'a T) -> S<'a>`
    pub fn compute_type_with_lifetime_twin_normal<'a>(
        &'a self,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        LtTypeWithLifetimeTwinNormal { field: self }
    }

    /// `fn f(x: &'a T) -> &'a S`
    pub fn compute_sub_struct_twin_normal<'a>(&'a self) -> &'a LtOwnedSubStructTwinNormal {
        &self.sub
    }

    /// The unrelated arg should not affect results
    pub fn compute_with_unrelated_borrowed_arg_twin_normal<'a>(
        &'a self,
        unrelated_borrowed: &LtOwnedSubStructTwinNormal,
        unrelated_owned: LtOwnedSubStructTwinNormal,
    ) -> &'a LtOwnedSubStructTwinNormal {
        assert_eq!(&unrelated_borrowed.value, "hi");
        assert_eq!(&unrelated_owned.value, "hi");
        &self.sub
    }
}

/// Functions (other tests are mainly methods)
pub fn lt_compute_with_lifetime_function_twin_normal<'a>(
    a: &'a LtOwnedStructTwinNormal,
) -> &'a LtOwnedSubStructTwinNormal {
    &a.sub
}

impl LtOwnedSubStructTwinNormal {
    pub fn greet_borrow_self_twin_normal(&self) -> String {
        self.value.clone()
    }

    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> String {
        self.value.clone()
    }
}

impl LtTypeWithLifetimeTwinNormal<'_> {
    /// `&T` where T is lifetimeable
    pub fn greet_borrow_self_twin_normal(&self) -> String {
        self.field.sub.value.clone()
    }

    /// `&mut T` where T is lifetimeable
    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> String {
        self.field.sub.value.clone()
    }

    /// Input lifetimeable and output another lifetimeable
    pub fn compute_double_type_with_lifetime_twin_normal<'a>(
        &'a self,
    ) -> LtNestedTypeWithLifetimeTwinNormal<'a> {
        LtNestedTypeWithLifetimeTwinNormal { field: self }
    }

    /// Input argument has type `T<'a>` (other tests mainly are `&'a T`
    pub fn compute_arg_generic_lifetime_twin_normal<'a>(
        arg: LtTypeWithLifetimeTwinNormal<'a>,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        LtTypeWithLifetimeTwinNormal { field: arg.field }
    }
}

impl LtNestedTypeWithLifetimeTwinNormal<'_> {
    pub fn greet_borrow_self_twin_normal(&self) -> String {
        self.field.field.sub.value.clone()
    }

    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> String {
        self.field.field.sub.value.clone()
    }
}

impl LtTypeWithMultiOwnerTwinNormal<'_> {
    /// Multiple input args have lifetime
    pub fn compute_with_multi_arg_having_lifetime_twin_normal<'a>(
        a: &'a LtOwnedStructTwinNormal,
        b: &'a LtOwnedStructTwinNormal,
        unrelated_borrowed: &LtOwnedSubStructTwinNormal,
        unrelated_owned: LtOwnedSubStructTwinNormal,
    ) -> Self<'a> {
        assert_eq!(&unrelated_borrowed.value, "hi");
        assert_eq!(&unrelated_owned.value, "hi");
        Self { fields: vec![a, b] }
    }

    pub fn greet_borrow_self_twin_normal(&self) -> Vec<String> {
        self.fields.iter().map(|x| x.sub.value.clone()).collect()
    }

    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> Vec<String> {
        self.fields.iter().map(|x| x.sub.value.clone()).collect()
    }
}
