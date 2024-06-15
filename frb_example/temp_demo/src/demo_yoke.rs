use crate::user_code::*;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use yoke::Yoke;

fn load_object(filename: &str) -> Yoke<RwLockReadGuard<'static, One>, Arc<RwLock<One>>> {
    let one: Arc<RwLock<One>> = Arc::new(RwLock::new(One("hi_one".to_owned())));
    Yoke::attach_to_cart(one, |one: &RwLock<One>| one.blocking_read())
}

pub fn main() {
    let yoke = load_object("filename.bincode");
    println!("yoke={yoke:?}");
    // TODO
    // assert_eq!(&**yoke.get(), [2u8, 3]);
    // assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
}
