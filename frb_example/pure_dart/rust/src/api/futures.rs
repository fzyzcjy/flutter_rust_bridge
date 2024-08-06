// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};


pub enum CustomErr{
    Failure
}

pub fn impl_future_adder(a: i32, b: i32) -> impl Future<Output = i32> {
    async {
        a + b
    }
}

pub fn impl_future_adder_result(a: i32, b: i32, succeed: bool) -> Pin<Box<dyn Future<Output = Result<i32, CustomErr>> + Send + 'static>> {
    Box::pin(async {
        match succeed {
            true => Ok(a + b),
            false => Err(CustomErr::Failure)
        }
    })
}

pub fn dartfn_future_adder(a: i32, b: i32, c: i32) -> DartFnFuture<i32> {
    Box::pin(async {
        a + b + c
    })
}

pub fn dartfn_future_adder(a: i32, b: i32, c: i32) -> DartFnFuture<Result<i32, CustomErr>> {
    Box::pin(async {
        match succeed {
            true => Ok(a + b + c),
            false => Err(CustomErr::Failure)
        }
    })
}

pub fn box_future_adder(a: i32, b: i32, c: i32, d: i32) -> Pin<Box<dyn Future<Output = i32> + Send + 'static>> {
    Box::pin(async {
        a + b + c + d
    })
}

pub fn box_future_adder_result(a: i32, b: i32, c: i32, d: i32, succeed: bool) -> Pin<Box<dyn Future<Output = Result<i32, CustomErr>> + Send + 'static>> {
    Box::pin(async {
        match succeed {
            true => Ok(a + b + c + d),
            false => Err(CustomErr::Failure)
        }
    })
}

#[frb(opaque)]
pub struct StructWithAsyncMethods {
    name: String
}

impl StructWithAsyncMethods {
    #[frb(sync)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }

    pub fn impl_future_hello(&self) -> impl Future<Output = Result<String, MyErr>> {
       async {
            format("Hello, {}", &self.name)
        }
    }

    pub fn dartfn_future_hello(&self) -> DartFnFuture<String> {
        Box::pin(async {
            format("Bonjour, {}", &self.name)
        })
    }

    pub fn box_future_hello(&self) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
        Box::pin(async {
            format("Hola, {}", &self.name)
        })
    }

    pub fn impl_future_hello_result(&self, succeed: bool) -> impl Future<Output = Result<String, MyErr>> {
       async {
            match succeed {
                true => Ok(format("Hello, {}", &self.name)),
                false => Err(CustomErr::Failure)
            }
        }
    }

    pub fn dartfn_future_hello_result(&self, succeed: bool) -> DartFnFuture<String> {
        Box::pin(async {
            match succeed {
                true => Ok(format("Bonjour, {}", &self.name)),
                false => Err(CustomErr::Failure)
            }
        })
    }

    pub fn box_future_hello_result(&self, succeed: bool) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
        Box::pin(async {
            match succeed {
                true => Ok(format("Hola, {}", &self.name)),
                false => Err(CustomErr::Failure)
            }
        })
    }

}

pub trait TraitWithAsyncMethods {
    fn example_async_method(
        &mut self,
        arg_one: u8,
        arg_two: u32,
        arg_three: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, CustomErr>>;

}