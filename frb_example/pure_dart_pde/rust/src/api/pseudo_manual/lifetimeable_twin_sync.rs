// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `lifetimeable.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "sse", "sync sse", "rustAsync sse"], "replaceCode": {"misc_no_twin_example_a": "misc_no_twin_example_a"}}

#![allow(clippy::needless_lifetimes)]

use crate::api::misc_no_twin_example_a::SimpleLogger;
use flutter_rust_bridge::frb;

// --------------------------- struct definitions ---------------------------

/// Try *NOT* to impl Clone for these types in order to ensure there are no extra clones
#[frb(opaque)]
#[derive(Debug)]
pub struct LtOwnedStructTwinSync {
    sub: LtSubStructTwinSync,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtSubStructTwinSync {
    value: String,
    logger: SimpleLogger,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtTypeWithLifetimeTwinSync<'a> {
    field: &'a LtOwnedStructTwinSync,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtNestedTypeWithLifetimeTwinSync<'a> {
    field: &'a LtTypeWithLifetimeTwinSync<'a>,
}

#[frb(opaque)]
#[derive(Debug)]
pub struct LtTypeWithMultiDepTwinSync<'a> {
    fields: Vec<&'a LtOwnedStructTwinSync>,
}

// --------------------------- drops ---------------------------

impl Drop for LtOwnedStructTwinSync {
    fn drop(&mut self) {
        self.sub.logger.log("LtOwnedStructTwinSync.drop");
    }
}

impl Drop for LtSubStructTwinSync {
    fn drop(&mut self) {
        self.logger.log("LtOwnedSubStructTwinSync.drop");
    }
}

impl Drop for LtTypeWithLifetimeTwinSync<'_> {
    fn drop(&mut self) {
        (self.field.sub.logger).log("LtTypeWithLifetimeTwinSync.drop");
    }
}

impl Drop for LtNestedTypeWithLifetimeTwinSync<'_> {
    fn drop(&mut self) {
        (self.field.field.sub.logger).log("LtNestedTypeWithLifetimeTwinSync.drop");
    }
}

// --------------------------- methods ---------------------------

impl LtOwnedStructTwinSync {
    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_twin_sync(value: String) -> Self {
        Self {
            sub: LtSubStructTwinSync {
                value,
                logger: SimpleLogger::new(),
            },
        }
    }

    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn create_with_logger_twin_sync(value: String, logger: &SimpleLogger) -> Self {
        Self {
            sub: LtSubStructTwinSync {
                value,
                logger: logger.clone(),
            },
        }
    }

    /// `fn f(x: &'a T) -> S<'a>`
    #[frb(serialize)]
    pub fn compute_type_with_lifetime_twin_sync<'a>(&'a self) -> LtTypeWithLifetimeTwinSync<'a> {
        LtTypeWithLifetimeTwinSync { field: self }
    }

    // /// `fn f(x: &'a T) -> &'a S`
    // pub fn compute_sub_struct_twin_sync<'a>(&'a self) -> &'a LtSubStructTwinSync {
    //     &self.sub
    // }

    /// The unrelated arg should not affect results
    #[frb(serialize)]
    pub fn compute_with_unrelated_borrowed_arg_twin_sync<'a>(
        &'a self,
        unrelated_borrowed: &LtOwnedStructTwinSync,
        unrelated_owned: LtOwnedStructTwinSync,
    ) -> LtTypeWithLifetimeTwinSync<'a> {
        assert_eq!(&unrelated_borrowed.sub.value, "hi");
        assert_eq!(&unrelated_owned.sub.value, "hi");
        LtTypeWithLifetimeTwinSync { field: self }
    }
}

/// Functions (other tests are mainly methods)
#[frb(serialize)]
pub fn lt_compute_with_lifetime_function_twin_sync<'a>(
    arg: &'a LtOwnedStructTwinSync,
) -> LtTypeWithLifetimeTwinSync<'a> {
    LtTypeWithLifetimeTwinSync { field: arg }
}

impl LtSubStructTwinSync {
    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_self_twin_sync(&self) -> String {
        self.value.clone()
    }

    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_mut_self_twin_sync(&mut self) -> String {
        self.value.clone()
    }
}

impl LtTypeWithLifetimeTwinSync<'_> {
    /// `&T` where T is lifetimeable
    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_self_twin_sync(&self) -> String {
        self.field.sub.value.clone()
    }

    /// `&mut T` where T is lifetimeable
    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_mut_self_twin_sync(&mut self) -> String {
        self.field.sub.value.clone()
    }

    /// Input lifetimeable and output another lifetimeable
    #[frb(serialize)]
    pub fn compute_nested_type_with_lifetime_twin_sync<'a>(
        &'a self,
    ) -> LtNestedTypeWithLifetimeTwinSync<'a> {
        LtNestedTypeWithLifetimeTwinSync { field: self }
    }

    /// Input argument has type `T<'a>` (other tests mainly are `&'a T`)
    #[frb(serialize)]
    pub fn compute_arg_generic_lifetime_twin_sync<'a>(
        arg: &LtTypeWithLifetimeTwinSync<'a>,
    ) -> LtTypeWithLifetimeTwinSync<'a> {
        LtTypeWithLifetimeTwinSync { field: arg.field }
    }
}

impl LtNestedTypeWithLifetimeTwinSync<'_> {
    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_self_twin_sync(&self) -> String {
        self.field.field.sub.value.clone()
    }

    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_mut_self_twin_sync(&mut self) -> String {
        self.field.field.sub.value.clone()
    }
}

impl LtTypeWithMultiDepTwinSync<'_> {
    /// Multiple input args have lifetime
    #[frb(serialize)]
    pub fn compute_with_multi_arg_having_lifetime_twin_sync<'a>(
        a: &'a LtOwnedStructTwinSync,
        b: &'a LtOwnedStructTwinSync,
        unrelated_borrowed: &LtOwnedStructTwinSync,
        unrelated_owned: LtOwnedStructTwinSync,
    ) -> LtTypeWithMultiDepTwinSync<'a> {
        assert_eq!(&unrelated_borrowed.sub.value, "hi");
        assert_eq!(&unrelated_owned.sub.value, "hi");
        LtTypeWithMultiDepTwinSync { fields: vec![a, b] }
    }

    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_self_twin_sync(&self) -> Vec<String> {
        self.fields.iter().map(|x| x.sub.value.clone()).collect()
    }

    #[frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn greet_borrow_mut_self_twin_sync(&mut self) -> Vec<String> {
        self.fields.iter().map(|x| x.sub.value.clone()).collect()
    }
}
