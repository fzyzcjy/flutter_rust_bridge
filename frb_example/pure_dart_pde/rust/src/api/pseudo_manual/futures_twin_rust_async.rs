// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `futures.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::{frb, DartFnFuture};
use std::future::Future;
use std::pin::Pin;

pub enum CustomErrTwinRustAsync {
    Failure,
}

pub async fn impl_future_adder_twin_rust_async(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn impl_future_adder_result_twin_rust_async(
    a: i32,
    b: i32,
    succeed: bool,
) -> Pin<Box<dyn Future<Output = Result<i32, CustomErrTwinRustAsync>> + Send + 'static>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b),
            false => Err(CustomErrTwinRustAsync::Failure),
        }
    })
}

pub async fn dartfn_future_adder_twin_rust_async(a: i32, b: i32, c: i32) -> DartFnFuture<i32> {
    Box::pin(async move { a + b + c })
}

pub async fn dartfn_future_adder_result_twin_rust_async(
    a: i32,
    b: i32,
    c: i32,
    succeed: bool,
) -> DartFnFuture<Result<i32, CustomErrTwinRustAsync>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b + c),
            false => Err(CustomErrTwinRustAsync::Failure),
        }
    })
}

pub async fn box_future_adder_twin_rust_async(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
) -> Pin<Box<dyn Future<Output = i32> + Send + 'static>> {
    Box::pin(async move { a + b + c + d })
}

pub async fn box_future_adder_result_twin_rust_async(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    succeed: bool,
) -> Pin<Box<dyn Future<Output = Result<i32, CustomErrTwinRustAsync>> + Send + 'static>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b + c + d),
            false => Err(CustomErrTwinRustAsync::Failure),
        }
    })
}

#[frb(opaque)]
pub struct StructWithAsyncMethodsTwinRustAsync {
    name: String,
}

impl StructWithAsyncMethodsTwinRustAsync {
    #[frb(sync)]
    pub async fn new_twin_rust_async(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub async fn impl_future_hello_twin_rust_async(&self) -> impl Future<Output = String> {
        let name = self.name.clone();
        async move { format!("Hello, {}", name) }
    }

    pub async fn impl_future_hello_result_twin_rust_async(
        &self,
        succeed: bool,
    ) -> impl Future<Output = Result<String, CustomErrTwinRustAsync>> {
        let name = self.name.clone();
        async move {
            match succeed {
                true => Ok(format!("Hello, {}", name)),
                false => Err(CustomErrTwinRustAsync::Failure),
            }
        }
    }

    pub async fn dartfn_future_hello_twin_rust_async(&self) -> DartFnFuture<String> {
        let name = self.name.clone();
        Box::pin(async move { format!("Bonjour, {}", name) })
    }

    pub async fn dartfn_future_hello_result_twin_rust_async(
        &self,
        succeed: bool,
    ) -> DartFnFuture<Result<String, CustomErrTwinRustAsync>> {
        let name = self.name.clone();
        Box::pin(async move {
            match succeed {
                true => Ok(format!("Bonjour, {}", name)),
                false => Err(CustomErrTwinRustAsync::Failure),
            }
        })
    }

    pub async fn box_future_hello_twin_rust_async(
        &self,
    ) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
        let name = self.name.clone();
        Box::pin(async move { format!("Hola, {}", name) })
    }

    pub async fn box_future_hello_result_twin_rust_async(
        &self,
        succeed: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, CustomErrTwinRustAsync>> + Send + 'static>>
    {
        let name = self.name.clone();
        Box::pin(async move {
            match succeed {
                true => Ok(format!("Hola, {}", name)),
                false => Err(CustomErrTwinRustAsync::Failure),
            }
        })
    }
}

pub trait TraitWithAsyncMethodsTwinRustAsync {
    fn example_async_method(
        &mut self,
        arg_one: u8,
        arg_two: u32,
        arg_three: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, CustomErrTwinRustAsync>>;
}
