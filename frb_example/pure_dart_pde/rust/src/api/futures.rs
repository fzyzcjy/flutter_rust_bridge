// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::{frb, DartFnFuture};
use std::future::Future;
use std::pin::Pin;

pub enum CustomErrTwinNormal {
    Failure,
}

pub async fn impl_future_adder_twin_normal(a: i32, b: i32) -> i32 {
    a + b
}

pub fn impl_future_adder_result_twin_normal(
    a: i32,
    b: i32,
    succeed: bool,
) -> Pin<Box<dyn Future<Output = Result<i32, CustomErrTwinNormal>> + Send + 'static>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b),
            false => Err(CustomErrTwinNormal::Failure),
        }
    })
}

pub fn dartfn_future_adder_twin_normal(a: i32, b: i32, c: i32) -> DartFnFuture<i32> {
    Box::pin(async move { a + b + c })
}

pub fn dartfn_future_adder_result_twin_normal(
    a: i32,
    b: i32,
    c: i32,
    succeed: bool,
) -> DartFnFuture<Result<i32, CustomErrTwinNormal>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b + c),
            false => Err(CustomErrTwinNormal::Failure),
        }
    })
}

pub fn box_future_adder_twin_normal(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
) -> Pin<Box<dyn Future<Output = i32> + Send + 'static>> {
    Box::pin(async move { a + b + c + d })
}

pub fn box_future_adder_result_twin_normal(
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    succeed: bool,
) -> Pin<Box<dyn Future<Output = Result<i32, CustomErrTwinNormal>> + Send + 'static>> {
    Box::pin(async move {
        match succeed {
            true => Ok(a + b + c + d),
            false => Err(CustomErrTwinNormal::Failure),
        }
    })
}

#[frb(opaque)]
pub struct StructWithAsyncMethodsTwinNormal {
    name: String,
}

impl StructWithAsyncMethodsTwinNormal {
    #[frb(sync)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn impl_future_hello(&self) -> impl Future<Output = String> {
        let name = self.name.clone();
        async move { format!("Hello, {}", name) }
    }

    pub fn impl_future_hello_result(
        &self,
        succeed: bool,
    ) -> impl Future<Output = Result<String, CustomErrTwinNormal>> {
        let name = self.name.clone();
        async move {
            match succeed {
                true => Ok(format!("Hello, {}", name)),
                false => Err(CustomErrTwinNormal::Failure),
            }
        }
    }

    pub fn dartfn_future_hello(&self) -> DartFnFuture<String> {
        let name = self.name.clone();
        Box::pin(async move { format!("Bonjour, {}", name) })
    }

    pub fn dartfn_future_hello_result(
        &self,
        succeed: bool,
    ) -> DartFnFuture<Result<String, CustomErrTwinNormal>> {
        let name = self.name.clone();
        Box::pin(async move {
            match succeed {
                true => Ok(format!("Bonjour, {}", name)),
                false => Err(CustomErrTwinNormal::Failure),
            }
        })
    }

    pub fn box_future_hello(&self) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
        let name = self.name.clone();
        Box::pin(async move { format!("Hola, {}", name) })
    }

    pub fn box_future_hello_result(
        &self,
        succeed: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, CustomErrTwinNormal>> + Send + 'static>> {
        let name = self.name.clone();
        Box::pin(async move {
            match succeed {
                true => Ok(format!("Hola, {}", name)),
                false => Err(CustomErrTwinNormal::Failure),
            }
        })
    }
}

pub trait TraitWithAsyncMethodsTwinNormal {
    fn example_async_method(
        &mut self,
        arg_one: u8,
        arg_two: u32,
        arg_three: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, CustomErrTwinNormal>>;
}
