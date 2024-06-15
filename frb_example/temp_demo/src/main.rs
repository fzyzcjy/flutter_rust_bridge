use self_cell::self_cell;
use std::sync::{Arc, RwLock};

#[derive(Debug)]
struct One(String);

#[derive(Debug)]
struct Two<'a> {
    one: &'a One,
}

self_cell!(
    struct Pack {
        owner: Arc<RwLock<One>>,

        #[covariant]
        dependent: Two,
    }

    impl {Debug}
);

fn build_pack() -> Pack {
    let one = Arc::new(RwLock::new(One("hello".to_owned())));
    Pack::try_new(one, |one| Ok::<Two, ()>(panic!())).unwrap()
}

fn main() {
    let pack = build_pack();

    println!("pack -> {:?}", &pack);
    println!("pack.borrow_owner() -> {:?}", pack.borrow_owner());
    println!("pack.borrow_dependent() -> {:?}", pack.borrow_dependent());
}
