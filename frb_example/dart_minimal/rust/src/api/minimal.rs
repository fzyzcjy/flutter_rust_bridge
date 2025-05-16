use flutter_rust_bridge::for_generated::IntoDartExceptPrimitive;
use flutter_rust_bridge::{frb, IntoDart, IntoIntoDart};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Clone, Copy, PartialEq)]
#[frb(sync)]
pub enum Entity {
    OptionA,
    OptionB,
}

#[derive(Clone, Copy, PartialEq)]
#[frb(sync)]
pub enum ElementKind {
    Empty,
    Occupied(Entity),
}

#[derive(Clone, Copy, PartialEq)]
#[frb(sync)]
pub struct StateType {
    pub area: [[ElementKind; 3]; 3],
}

impl StateType {
    pub fn f(self) {}
}

pub fn f_intodart<T: IntoDart>() {}
pub fn f_intointodart<T: IntoIntoDart<X>, X: IntoDart>() {}

pub fn g() {
    f_intodart::<[ElementKind; 3]>();
    f_intointodart::<[[ElementKind; 3]; 3], [[ElementKind; 3]; 3]>();
}
