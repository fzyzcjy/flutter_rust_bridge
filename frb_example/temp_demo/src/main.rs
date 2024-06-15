use self_cell::self_cell;

#[derive(Debug)]
struct One(String);

#[derive(Debug)]
struct Two<'a> {
    one: &'a One,
}

self_cell!(
    struct Pack {
        owner: One,

        #[covariant]
        dependent: Two,
    }

    impl {Debug}
);

fn build_pack() -> Pack {
    let one = One("hello".to_owned());
    Pack::new(one, |one| Two { one })
}

fn main() {
    let pack = build_pack();

    println!("pack -> {:?}", &pack);
    println!("pack.borrow_owner() -> {:?}", pack.borrow_owner());
    println!("pack.borrow_dependent() -> {:?}", pack.borrow_dependent());
}
