#[derive(Debug, Clone)]
pub struct StructSimpleTwinNormal {
    pub width: i32,
    pub height: i32,
}

pub fn func_struct_simple_twin_normal(arg: StructSimpleTwinNormal) -> StructSimpleTwinNormal {
    arg
}

pub fn func_struct_simple_boxed_twin_normal(
    arg: Box<StructSimpleTwinNormal>,
) -> Box<StructSimpleTwinNormal> {
    arg
}
