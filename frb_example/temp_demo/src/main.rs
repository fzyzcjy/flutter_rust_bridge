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

fn build_pack() -> OneAndGuardAndTwo {
    let one = Arc::new(RwLock::new(One("hello".to_owned())));
    let one_and_guard = OneAndGuard::try_new(one, |one| Ok::<_, ()>(one.read().unwrap())).unwrap();
    OneAndGuardAndTwo::try_new(one_and_guard, |one_and_guard| {
        Ok::<_, ()>(Two {
            one: one_and_guard.borrow_dependent(),
        })
    })
    .unwrap()
}

fn main() {
    let pack = build_pack();

    println!("pack -> {:?}", &pack);
    println!("pack.borrow_owner() -> {:?}", pack.borrow_owner());
    println!("pack.borrow_dependent() -> {:?}", pack.borrow_dependent());
}
