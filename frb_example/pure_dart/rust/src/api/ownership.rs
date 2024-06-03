pub fn borrow_string_twin_normal(arg: &String) -> String {
    arg.to_owned()
}

pub fn borrow_str_twin_normal(arg: &str) -> &str {
    arg
}

pub fn borrow_i32_twin_normal(arg: &i32) -> i32 {
    *arg
}

#[derive(Clone)]
pub struct SimpleStructForBorrowTwinNormal {
    pub one: String,
}

pub fn borrow_struct_twin_normal(
    arg: &SimpleStructForBorrowTwinNormal,
) -> SimpleStructForBorrowTwinNormal {
    arg.clone()
}
