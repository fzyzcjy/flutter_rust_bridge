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
type YokeVecTwoWrapped = Yoke<Vec<TwoWrapped<'static>>, Arc<YokeGuardOne>>;

fn compute_guard(one: Arc<RwLock<One>>) -> Arc<YokeGuardOne> {
    Arc::new(Yoke::attach_to_cart(one, |one: &RwLock<One>| {
        RwLockReadGuardOne(one.blocking_read())
    }))
}

// fn compute_two(guard_one: Arc<YokeGuardOne>) -> YokeTwoWrapped {
//     Yoke::attach_to_cart(guard_one, |guard_one: &YokeGuardOne| {
//         TwoWrapped(Two { one: guard_one.get().0.deref(), unrelated: "".to_string() })
//     })
// }

fn compute_vec_two(guard_one: Arc<YokeGuardOne>) -> YokeVecTwoWrapped {
    Yoke::attach_to_cart(guard_one, |guard_one: &YokeGuardOne| {
        vec![
            TwoWrapped(Two { one: guard_one.get().0.deref(), unrelated: "item1".to_string() }),
            TwoWrapped(Two { one: guard_one.get().0.deref(), unrelated: "item2".to_string() }),
        ]
    })
}

pub fn main() {
    let one: Arc<RwLock<One>> = Arc::new(RwLock::new(One("hi_one".to_owned())));
    let guard = compute_guard(one.clone());
    let two = compute_vec_two(guard.clone());
    println!("one={one:?}");
    println!("guard={guard:?}");
    println!("two={two:?}");

    drop(one);
    drop(guard);
    println!("two(after drop others)={two:?}");

    // TODO
    // assert_eq!(&**yoke.get(), [2u8, 3]);
    // assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
}
