#![allow(unused_variables)]

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
pub use std::sync::{Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Result};

use backtrace::Backtrace;
use flutter_rust_bridge::*;
use lazy_static::lazy_static;

use crate::data::{EnumAlias, Id, MyEnum, MyStruct, StructAlias, UserIdAlias};
pub use crate::data::{
    FrbOpaqueReturn, FrbOpaqueSyncReturn, HideData, NonCloneData, NonSendHideData,
};
use log::info;

#[cfg(target_family = "wasm")]
mod helpers;

/// Some initialization code to run when the library is first loaded.
#[cfg(not(target_family = "wasm"))]
#[static_init::constructor]
extern "C" fn on_dylib_start() {
    _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .try_init();
}

#[derive(Debug, Clone)]
pub struct Log {
    pub key: u32,
    pub value: u32,
}
