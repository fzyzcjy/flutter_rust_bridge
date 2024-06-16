#![allow(clippy::needless_lifetimes)]

use flutter_rust_bridge::for_generated::lazy_static;
use flutter_rust_bridge::frb;
use std::sync::{Arc, Mutex};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// -------------------------------------------------------------------------------------------------

// TODO move this to a separate file that has *no* auto twin

#[derive(Debug, Clone)]
#[frb(opaque)]
pub struct SimpleLogger(Arc<Mutex<Vec<String>>>);

impl SimpleLogger {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(vec![])))
    }

    pub(crate) fn log(&self, message: &str) {
        self.0.lock().unwrap().push(message.to_owned());
    }

    pub fn get(&self) -> Vec<String> {
        self.0.lock().unwrap().drain(..).collect()
    }
}

// -------------------------------------------------------------------------------------------------

// --------------------------- struct definitions ---------------------------

/// Try *NOT* to impl Clone for these types in order to ensure there are no extra clones
#[frb(opaque)]
#[derive(Debug)]
pub struct LtOwnedStructTwinNormal {
    sub: LtSubStructTwinNormal,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtSubStructTwinNormal {
    value: String,
    logger: SimpleLogger,
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
pub struct LtTypeWithMultiDepTwinNormal<'a> {
    fields: Vec<&'a LtOwnedStructTwinNormal>,
}

// --------------------------- drops ---------------------------

impl Drop for LtOwnedStructTwinNormal {
    fn drop(&mut self) {
        self.sub.logger.log("LtOwnedStructTwinNormal.drop");
    }
}

impl Drop for LtSubStructTwinNormal {
    fn drop(&mut self) {
        self.logger.log("LtOwnedSubStructTwinNormal.drop");
    }
}

impl Drop for LtTypeWithLifetimeTwinNormal {
    fn drop(&mut self) {
        (self.field.sub.logger).log("LtTypeWithLifetimeTwinNormal.drop");
    }
}

impl Drop for LtNestedTypeWithLifetimeTwinNormal {
    fn drop(&mut self) {
        (self.field.field.sub.logger).log("LtNestedTypeWithLifetimeTwinNormal.drop");
    }
}

// --------------------------- methods ---------------------------

impl LtOwnedStructTwinNormal {
    pub fn create(value: String, logger: &SimpleLogger) -> Self {
        Self {
            sub: LtSubStructTwinNormal {
                value,
                logger: logger.clone(),
            },
        }
    }

    /// `fn f(x: &'a T) -> S<'a>`
    pub fn compute_type_with_lifetime_twin_normal<'a>(
        &'a self,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        LtTypeWithLifetimeTwinNormal { field: self }
    }

    // /// `fn f(x: &'a T) -> &'a S`
    // pub fn compute_sub_struct_twin_normal<'a>(&'a self) -> &'a LtSubStructTwinNormal {
    //     &self.sub
    // }

    /// The unrelated arg should not affect results
    pub fn compute_with_unrelated_borrowed_arg_twin_normal<'a>(
        &'a self,
        unrelated_borrowed: &LtSubStructTwinNormal,
        unrelated_owned: LtSubStructTwinNormal,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        assert_eq!(&unrelated_borrowed.value, "hi");
        assert_eq!(&unrelated_owned.value, "hi");
        LtTypeWithLifetimeTwinNormal { field: self }
    }
}

/// Functions (other tests are mainly methods)
pub fn lt_compute_with_lifetime_function_twin_normal<'a>(
    arg: &'a LtOwnedStructTwinNormal,
) -> LtTypeWithLifetimeTwinNormal<'a> {
    LtTypeWithLifetimeTwinNormal { field: arg }
}

impl LtSubStructTwinNormal {
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
    pub fn compute_nested_type_with_lifetime_twin_normal<'a>(
        &'a self,
    ) -> LtNestedTypeWithLifetimeTwinNormal<'a> {
        LtNestedTypeWithLifetimeTwinNormal { field: self }
    }

    /// Input argument has type `T<'a>` (other tests mainly are `&'a T`)
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

impl LtTypeWithMultiDepTwinNormal<'_> {
    /// Multiple input args have lifetime
    pub fn compute_with_multi_arg_having_lifetime_twin_normal<'a>(
        a: &'a LtOwnedStructTwinNormal,
        b: &'a LtOwnedStructTwinNormal,
        unrelated_borrowed: &LtSubStructTwinNormal,
        unrelated_owned: LtSubStructTwinNormal,
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
