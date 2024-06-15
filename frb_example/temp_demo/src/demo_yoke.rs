use crate::user_code::*;
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};
use yoke::{Yoke, Yokeable};

#[derive(Yokeable, Debug)]
struct RwLockReadGuardOne<'a>(RwLockReadGuard<'a, One>);

#[derive(Yokeable, Debug)]
struct TwoWrapped<'a>(Two<'a>);

type YokeGuardOne = Yoke<RwLockReadGuardOne<'static>, Arc<RwLock<One>>>;
type YokeTwoWrapped = Yoke<TwoWrapped<'static>, Arc<YokeGuardOne>>;

fn compute_guard(one: Arc<RwLock<One>>) -> Arc<YokeGuardOne> {
    Arc::new(Yoke::attach_to_cart(one, |one: &RwLock<One>| {
        RwLockReadGuardOne(one.blocking_read())
    }))
}

fn compute_two(guard_one: Arc<YokeGuardOne>) -> YokeTwoWrapped {
    Yoke::attach_to_cart(guard_one, |guard_one: &YokeGuardOne| {
        TwoWrapped(Two { one: guard_one.get().0.deref(), unrelated: "".to_string() })
    })
}

pub fn main() {
    let one: Arc<RwLock<One>> = Arc::new(RwLock::new(One("hi_one".to_owned())));
    let guard = compute_guard(one.clone());
    let two = compute_two(guard.clone());
    println!("one={one:?}");
    println!("guard={guard:?}");
    println!("two={two:?}");
    // TODO
    // assert_eq!(&**yoke.get(), [2u8, 3]);
    // assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
}
