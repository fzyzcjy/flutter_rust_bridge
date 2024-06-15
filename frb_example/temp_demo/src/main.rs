use self_cell::self_cell;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[derive(Debug)]
struct One(String);

#[derive(Debug)]
struct Two<'a> {
    one: &'a One,
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

fn build_pack(one: Arc<RwLock<One>>) -> anyhow::Result<OneAndGuardAndTwo> {
    let one_and_guard = OneAndGuard::try_new(one, |one| {
        one.read().map_err(|_| anyhow::anyhow!("read lock failed"))
    })?;
    let one_and_guard_and_two = OneAndGuardAndTwo::try_new(one_and_guard, |one_and_guard| {
        anyhow::Ok(Two {
            one: one_and_guard.borrow_dependent(),
        })
    })?;
    Ok(one_and_guard_and_two)
}

fn main() -> anyhow::Result<()> {
    let one = Arc::new(RwLock::new(One("hello".to_owned())));
    let pack = build_pack(one.clone())?;

    println!("one(cloned) -> {:?}", &one);
    println!("pack -> {:?}", &pack);
    println!("pack.borrow_owner() -> {:?}", pack.borrow_owner());
    println!("pack.borrow_dependent() -> {:?}", pack.borrow_dependent());

    Ok(())
}
