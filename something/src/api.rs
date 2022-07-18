pub enum CustomError {
    Error1(String),
    Error2(u32),
    Error3(i32),
}

/*
pub fn return_custom() -> CustomError {
    todo!()
}
*/

pub fn return_custom_error() -> Result<i32, CustomError> {
    Err(CustomError::Error2(3))
}
