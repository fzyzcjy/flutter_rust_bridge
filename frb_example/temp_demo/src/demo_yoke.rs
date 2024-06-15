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

#[derive(Yokeable, Debug)]
struct VecTwoWrapped<'a>(Vec<Two<'a>>);

type YokeGuardOne = Yoke<RwLockReadGuardOne<'static>, Arc<RwLock<One>>>;
type YokeTwoWrapped = Yoke<TwoWrapped<'static>, Arc<YokeGuardOne>>;
type YokeVecTwoWrapped = Yoke<VecTwoWrapped<'static>, Arc<YokeGuardOne>>;

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
        VecTwoWrapped(vec![
            Two {
                one: guard_one.get().0.deref(),
                unrelated: "item1".to_string(),
            },
            Two {
                one: guard_one.get().0.deref(),
                unrelated: "item2".to_string(),
            },
        ])
    })
}

fn split_vec_two(yoke_vec_two_wrapped: YokeVecTwoWrapped) -> Vec<YokeTwoWrapped> {
    let len = yoke_vec_two_wrapped.get().0.len();
    ((0..len).into_iter())
        .map(|index| {
            yoke_vec_two_wrapped.map_project_cloned(|x| x.0[index])
        })
        .collect()
}

pub fn main() {
    let one: Arc<RwLock<One>> = Arc::new(RwLock::new(One("hi_one".to_owned())));
    println!("one={one:?}");

    let guard = compute_guard(one.clone());
    println!("guard={guard:?}");

    let vec_two_raw = compute_vec_two(guard.clone());
    println!("vec_two_raw={vec_two_raw:?}");

    let vec_two_split = split_vec_two(vec_two_raw);
    println!("vec_two_split={vec_two_split:?}");

    drop(one);
    drop(guard);
    println!("vec_two_split(after drop others)={vec_two_split:?}");

    // TODO
    // assert_eq!(&**yoke.get(), [2u8, 3]);
    // assert!(matches!(yoke.get(), &Cow::Borrowed(_)));
}
