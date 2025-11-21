use flutter_rust_bridge::frb;

pub struct Container<T> {
    pub value: T,
    pub label: String,
}

pub type StringContainer = Container<String>;
pub type IntContainer = Container<i32>;

pub fn func_string_container(arg: StringContainer) -> StringContainer {
    arg
}
pub fn func_int_container(arg: IntContainer) -> IntContainer {
    arg
}

#[frb(ignore)]
pub struct IgnoredWrapper<T> {
    pub inner: T,
}

#[frb(non_opaque)]
pub type VisibleStringWrapper = IgnoredWrapper<String>;

pub fn func_visible_string_wrapper(arg: VisibleStringWrapper) -> VisibleStringWrapper {
    arg
}
