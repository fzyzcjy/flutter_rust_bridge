use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(opaque)]
pub struct Foo(String);

#[frb(opaque)]
pub struct Bar<'a>(&'a Foo);

#[allow(clippy::needless_lifetimes)]
impl Foo {
    #[frb(sync)]
    pub fn new() -> Self {
        Self("hello".to_owned())
    }

    pub fn compute_bar<'a>(&'a self) -> Bar<'a> {
        Bar(self)
    }
}
