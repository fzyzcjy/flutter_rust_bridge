pub enum AppError {
    Bad,
}

// Generic alias over a fallible `Result`: `AppResult<i32>` must expand to
// `Result<i32, AppError>` at the use site (see #3071).
pub type AppResult<T> = std::result::Result<T, AppError>;

pub fn generic_alias_result(input: i32) -> AppResult<i32> {
    Ok(input)
}

// Generic alias over a non-`Result` type: `Wrapper<i32>` must expand to
// `Option<i32>` at the use site.
pub type Wrapper<T> = Option<T>;

pub fn generic_alias_option(input: i32) -> Wrapper<i32> {
    Some(input)
}
