// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `futures.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::{frb, DartFnFuture};
use std::future::Future;
use std::pin::Pin;

pub enum CustomErrTwinSse {
    Failure,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn impl_future_adder_twin_sse(a: i32, b: i32) -> i32 {
    a + b
}

#[flutter_rust_bridge::frb(serialize)]
pub fn impl_future_adder_result_twin_sse(
    a: i32,
    b: i32,
    succeed: bool,
) -> Pin<Box<dyn Future<Output = Result<i32, CustomErrTwinSse>> + Send + 'static>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b),
            false => Err(CustomErrTwinSse::Failure),
        }
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn dartfn_future_adder_twin_sse(a: i32, b: i32, c: i32) -> DartFnFuture<i32> {
    Box::pin(async move { a + b + c })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn dartfn_future_adder_result_twin_sse(
    a: i32,
    b: i32,
    c: i32,
    succeed: bool,
) -> DartFnFuture<Result<i32, CustomErrTwinSse>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b + c),
            false => Err(CustomErrTwinSse::Failure),
        }
    })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn box_future_adder_twin_sse(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
) -> Pin<Box<dyn Future<Output = i32> + Send + 'static>> {
    Box::pin(async move { a + b + c + d })
}

#[flutter_rust_bridge::frb(serialize)]
pub fn box_future_adder_result_twin_sse(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    succeed: bool,
) -> Pin<Box<dyn Future<Output = Result<i32, CustomErrTwinSse>> + Send + 'static>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b + c + d),
            false => Err(CustomErrTwinSse::Failure),
        }
    })
}

#[frb(opaque)]
pub struct StructWithAsyncMethodsTwinSse {
    name: String,
}

impl StructWithAsyncMethodsTwinSse {
    #[frb(sync)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn impl_future_hello_twin_sse(&self) -> impl Future<Output = String> {
        let name = self.name.clone();
        async move { format!("Hello, {}", name) }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn impl_future_hello_result_twin_sse(
        &self,
        succeed: bool,
    ) -> impl Future<Output = Result<String, CustomErrTwinSse>> {
        let name = self.name.clone();
        async move {
            match succeed {
                true => Ok(format!("Hello, {}", name)),
                false => Err(CustomErrTwinSse::Failure),
            }
        }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn dartfn_future_hello_twin_sse(&self) -> DartFnFuture<String> {
        let name = self.name.clone();
        Box::pin(async move { format!("Bonjour, {}", name) })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn dartfn_future_hello_result_twin_sse(
        &self,
        succeed: bool,
    ) -> DartFnFuture<Result<String, CustomErrTwinSse>> {
        let name = self.name.clone();
        Box::pin(async move {
            match succeed {
                true => Ok(format!("Bonjour, {}", name)),
                false => Err(CustomErrTwinSse::Failure),
            }
        })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn box_future_hello_twin_sse(
        &self,
    ) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
        let name = self.name.clone();
        Box::pin(async move { format!("Hola, {}", name) })
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn box_future_hello_result_twin_sse(
        &self,
        succeed: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, CustomErrTwinSse>> + Send + 'static>> {
        let name = self.name.clone();
        Box::pin(async move {
            match succeed {
                true => Ok(format!("Hola, {}", name)),
                false => Err(CustomErrTwinSse::Failure),
            }
        })
    }
}

pub trait TraitWithAsyncMethodsTwinSse {
    fn example_async_method(
        &mut self,
        arg_one: u8,
        arg_two: u32,
        arg_three: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, CustomErrTwinSse>>;
}
