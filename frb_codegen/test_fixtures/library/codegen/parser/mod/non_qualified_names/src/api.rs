use anyhow::Result;
use chrono::*;
use uuid::Uuid;

pub fn func_1(duration: Duration, uuid: Uuid) {}

pub fn func_anyhow_result() -> Result<i32, String> {
    panic!()
}
