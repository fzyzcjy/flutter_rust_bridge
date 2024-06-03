pub fn borrow_string_twin_normal(arg: &String) -> String {
    arg.to_owned()
}

pub fn borrow_str_twin_normal(arg: &str) -> String {
    arg.to_owned()
}

pub fn borrow_i32_twin_normal(arg: &i32) -> i32 {
    *arg
}

pub fn borrow_slice_u8_twin_normal(arg: &[u8]) -> Vec<u8> {
    arg.to_owned()
}

pub fn borrow_slice_string_twin_normal(arg: &[String]) -> Vec<String> {
    arg.to_owned()
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
