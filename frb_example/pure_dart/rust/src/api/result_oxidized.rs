use flutter_rust_bridge::frb;

#[derive(Debug, Clone)]
pub struct ResultOxidizedError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ResultOxidizedUser {
    pub id: i32,
    pub name: String,
}

pub type WResult<T> = std::result::Result<T, ResultOxidizedError>;

pub type MyPair<A, B> = (A, B);

#[frb(oxidized)]
pub fn fallible_divide_oxidized(
    a: i32,
    b: i32,
) -> Result<i32, ResultOxidizedError> {
    divide_impl(a, b)
}

pub fn fallible_divide_throwing(a: i32, b: i32) -> Result<i32, ResultOxidizedError> {
    divide_impl(a, b)
}

#[frb(oxidized)]
#[frb(sync)]
pub fn fallible_divide_oxidized_sync(
    a: i32,
    b: i32,
) -> Result<i32, ResultOxidizedError> {
    divide_impl(a, b)
}

#[frb(oxidized)]
pub fn panic_oxidized_result() -> Result<i32, ResultOxidizedError> {
    panic!("deliberate oxidized panic")
}

#[frb(oxidized)]
#[frb(sync)]
pub fn wresult_alias_sync(a: i32, b: i32) -> WResult<i32> {
    divide_impl(a, b)
}

#[frb(oxidized)]
pub fn wresult_uuid() -> WResult<uuid::Uuid> {
    Ok(uuid::Uuid::from_u128(0x12345678123456781234567812345678))
}

#[frb(oxidized)]
#[frb(sync)]
pub fn wresult_string_sync(name: String) -> WResult<String> {
    if name.is_empty() {
        Err(error("name cannot be empty"))
    } else {
        Ok(format!("Hello, {name}!"))
    }
}

#[frb(oxidized)]
pub fn wresult_struct(id: i32, name: String) -> WResult<ResultOxidizedUser> {
    if id < 0 {
        Err(error("id must be non-negative"))
    } else {
        Ok(ResultOxidizedUser { id, name })
    }
}

#[frb(oxidized)]
pub fn wresult_vec(count: i32) -> WResult<Vec<i32>> {
    if count < 0 {
        Err(error("count must be non-negative"))
    } else {
        Ok((0..count).collect())
    }
}

#[frb(oxidized)]
pub fn wresult_option(value: Option<i32>) -> WResult<Option<String>> {
    match value {
        Some(value) if value < 0 => Err(error("value must be non-negative")),
        Some(value) => Ok(Some(format!("Value: {value}"))),
        None => Ok(None),
    }
}

#[frb(sync)]
pub fn pair_alias_sync(a: i32, b: String) -> MyPair<i32, String> {
    (a, b)
}

#[frb(oxidized)]
pub fn wresult_nested(items: Vec<String>) -> WResult<Vec<String>> {
    if items.is_empty() {
        Err(error("items cannot be empty"))
    } else {
        Ok(items.into_iter().map(|item| item.to_uppercase()).collect())
    }
}

fn divide_impl(a: i32, b: i32) -> Result<i32, ResultOxidizedError> {
    if b == 0 {
        Err(error("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn error(message: &str) -> ResultOxidizedError {
    ResultOxidizedError {
        message: message.to_owned(),
    }
}
