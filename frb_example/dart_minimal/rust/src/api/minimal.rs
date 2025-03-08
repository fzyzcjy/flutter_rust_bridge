use flutter_rust_bridge::frb;
pub use loro::ValueOrContainer;

#[frb(unignore, non_opaque)]
#[derive(Clone)]
pub struct Foo {
    bar: ValueOrContainer,
}
