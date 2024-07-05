use flutter_rust_bridge::frb;
pub use flutter_rust_bridge::DartFnFuture;
pub use std::future::Future;
pub use std::pin::Pin;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct MyStruct {}

impl MyStruct {
    pub fn example_static_method() {}

    pub fn example_instance_method(&self) {}

    pub fn example_async_method(&mut self) -> impl Future<Output = Result<String, MyErr>> {
       async {
            "result_value".to_string()
        }
    }

    pub fn example_async_future(&mut self) -> DartFnFuture<String> {
        async {
            "result_value".to_string()
        }
    }

    pub fn example_async_future2(&mut self) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
        async {
            "result_value".to_string()
        }
    }

}

pub enum MyErr{
    Oops,
}

pub trait MyAsyncTrait {
    fn example_async_method(
        &mut self,
        arg_one: u8,
        arg_two: u32,
        arg_three: Vec<u8>,
    ) -> impl Future<Output = Result<Vec<u8>, MyErr>>;

}
