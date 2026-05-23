// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref INIT_ONE_DONE: Mutex<bool> = Mutex::new(false);
    static ref INIT_TWO_DONE: Mutex<bool> = Mutex::new(false);
}

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[frb(init)]
pub fn my_init_one() -> anyhow::Result<()> {
    *INIT_ONE_DONE.lock().unwrap() = true;
    Ok(())
}

#[frb(init)]
pub fn my_init_two() {
    *INIT_TWO_DONE.lock().unwrap() = true;
}

pub fn check_init_done() -> bool {
    *INIT_ONE_DONE.lock().unwrap() && *INIT_TWO_DONE.lock().unwrap()
}
