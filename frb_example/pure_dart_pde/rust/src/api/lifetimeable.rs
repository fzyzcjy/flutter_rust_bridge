// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "sse", "sync sse", "rustAsync sse"], "replaceCode": {"pseudo_manual::misc_no_twin_example_a_twin_sync": "misc_no_twin_example_a"}}

#![allow(clippy::needless_lifetimes)]

use crate::api::misc_no_twin_example_a::SimpleLogger;
use flutter_rust_bridge::frb;

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

impl Drop for LtTypeWithLifetimeTwinNormal<'_> {
    fn drop(&mut self) {
        (self.field.sub.logger).log("LtTypeWithLifetimeTwinNormal.drop");
    }
}

impl Drop for LtNestedTypeWithLifetimeTwinNormal<'_> {
    fn drop(&mut self) {
        (self.field.field.sub.logger).log("LtNestedTypeWithLifetimeTwinNormal.drop");
    }
}

// --------------------------- methods ---------------------------

impl LtOwnedStructTwinNormal {
    #[frb(serialize)]
    pub fn create_twin_normal(value: String) -> Self {
        Self {
            sub: LtSubStructTwinNormal {
                value,
                logger: SimpleLogger::new(),
            },
        }
    }

    #[frb(serialize)]
    pub fn create_with_logger_twin_normal(value: String, logger: &SimpleLogger) -> Self {
        Self {
            sub: LtSubStructTwinNormal {
                value,
                logger: logger.clone(),
            },
        }
    }

    /// `fn f(x: &'a T) -> S<'a>`
    #[frb(serialize)]
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
    #[frb(serialize)]
    pub fn compute_with_unrelated_borrowed_arg_twin_normal<'a>(
        &'a self,
        unrelated_borrowed: &LtOwnedStructTwinNormal,
        unrelated_owned: LtOwnedStructTwinNormal,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        assert_eq!(&unrelated_borrowed.sub.value, "hi");
        assert_eq!(&unrelated_owned.sub.value, "hi");
        LtTypeWithLifetimeTwinNormal { field: self }
    }
}

/// Functions (other tests are mainly methods)
#[frb(serialize)]
pub fn lt_compute_with_lifetime_function_twin_normal<'a>(
    arg: &'a LtOwnedStructTwinNormal,
) -> LtTypeWithLifetimeTwinNormal<'a> {
    LtTypeWithLifetimeTwinNormal { field: arg }
}

impl LtSubStructTwinNormal {
    #[frb(serialize)]
    pub fn greet_borrow_self_twin_normal(&self) -> String {
        self.value.clone()
    }

    #[frb(serialize)]
    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> String {
        self.value.clone()
    }
}

impl LtTypeWithLifetimeTwinNormal<'_> {
    /// `&T` where T is lifetimeable
    #[frb(serialize)]
    pub fn greet_borrow_self_twin_normal(&self) -> String {
        self.field.sub.value.clone()
    }

    /// `&mut T` where T is lifetimeable
    #[frb(serialize)]
    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> String {
        self.field.sub.value.clone()
    }

    /// Input lifetimeable and output another lifetimeable
    #[frb(serialize)]
    pub fn compute_nested_type_with_lifetime_twin_normal<'a>(
        &'a self,
    ) -> LtNestedTypeWithLifetimeTwinNormal<'a> {
        LtNestedTypeWithLifetimeTwinNormal { field: self }
    }

    /// Input argument has type `T<'a>` (other tests mainly are `&'a T`)
    #[frb(serialize)]
    pub fn compute_arg_generic_lifetime_twin_normal<'a>(
        arg: &LtTypeWithLifetimeTwinNormal<'a>,
    ) -> LtTypeWithLifetimeTwinNormal<'a> {
        LtTypeWithLifetimeTwinNormal { field: arg.field }
    }
}

impl LtNestedTypeWithLifetimeTwinNormal<'_> {
    #[frb(serialize)]
    pub fn greet_borrow_self_twin_normal(&self) -> String {
        self.field.field.sub.value.clone()
    }

    #[frb(serialize)]
    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> String {
        self.field.field.sub.value.clone()
    }
}

impl LtTypeWithMultiDepTwinNormal<'_> {
    /// Multiple input args have lifetime
    #[frb(serialize)]
    pub fn compute_with_multi_arg_having_lifetime_twin_normal<'a>(
        a: &'a LtOwnedStructTwinNormal,
        b: &'a LtOwnedStructTwinNormal,
        unrelated_borrowed: &LtOwnedStructTwinNormal,
        unrelated_owned: LtOwnedStructTwinNormal,
    ) -> LtTypeWithMultiDepTwinNormal<'a> {
        assert_eq!(&unrelated_borrowed.sub.value, "hi");
        assert_eq!(&unrelated_owned.sub.value, "hi");
        LtTypeWithMultiDepTwinNormal { fields: vec![a, b] }
    }

    #[frb(serialize)]
    pub fn greet_borrow_self_twin_normal(&self) -> Vec<String> {
        self.fields.iter().map(|x| x.sub.value.clone()).collect()
    }

    #[frb(serialize)]
    pub fn greet_borrow_mut_self_twin_normal(&mut self) -> Vec<String> {
        self.fields.iter().map(|x| x.sub.value.clone()).collect()
    }
}
