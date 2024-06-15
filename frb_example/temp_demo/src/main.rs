use self_cell::self_cell;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
struct One(String);

#[derive(Debug)]
struct Two<'a> {
    one: &'a One,
    unrelated: i32,
}

type RwLockReadGuardOne<'a> = RwLockReadGuard<'a, One>;

self_cell!(
    struct OneAndGuard {
        owner: Arc<RwLock<One>>,

        #[covariant]
        dependent: RwLockReadGuardOne,
    }

    impl {Debug}
);

self_cell!(
    struct OneAndGuardAndTwo {
        owner: OneAndGuard,

        #[covariant]
        dependent: Two,
    }

    impl {Debug}
);

fn compute<'a>(one: &'a One, unrelated: &i32) -> Two<'a> {
    Two {
        one,
        unrelated: *unrelated,
    }
}

fn build_pack(
    one: Arc<RwLock<One>>,
    unrelated: Arc<RwLock<i32>>,
) -> anyhow::Result<OneAndGuardAndTwo> {
    let mut unrelated_guard: Option<RwLockReadGuard<i32>> = None;
    let one_and_guard = OneAndGuard::try_new(one, |one| {
        unrelated_guard = Some(unrelated.read().unwrap());
        one.read().map_err(|_| anyhow::anyhow!("read lock failed"))
    })?;
    let one_and_guard_and_two = OneAndGuardAndTwo::try_new(one_and_guard, |one_and_guard| {
        anyhow::Ok(compute(
            one_and_guard.borrow_dependent(),
            &*unrelated_guard.unwrap(),
        ))
    })?;
    Ok(one_and_guard_and_two)
}

fn main() -> anyhow::Result<()> {
    let one = Arc::new(RwLock::new(One("hello".to_owned())));
    let unrelated = Arc::new(RwLock::new(12345));
    let pack = build_pack(one.clone(), unrelated.clone())?;

    println!("one(cloned) -> {:?}", &one);
    println!("pack -> {:?}", &pack);
    println!("pack.borrow_owner() -> {:?}", pack.borrow_owner());
    println!("pack.borrow_dependent() -> {:?}", pack.borrow_dependent());

    Ok(())
}
