// use self_cell::self_cell;
// use std::sync::Arc;
// use tokio::sync::{RwLock, RwLockReadGuard};
//
// #[derive(Debug)]
// struct One(String);
//
// #[derive(Debug)]
// struct Unrelated(String);
//
// #[derive(Debug)]
// struct Two<'a> {
//     one: &'a One,
//     // test: what if unrelated is indeed an owner?
//     unrelated: &'a Unrelated,
//     // unrelated: String,
// }
//
// type RwLockReadGuardOne<'a> = RwLockReadGuard<'a, One>;
//
// self_cell!(
//     struct OneAndGuard {
//         owner: Arc<RwLock<One>>,
//
//         #[covariant]
//         dependent: RwLockReadGuardOne,
//     }
//
//     impl {Debug}
// );
//
// self_cell!(
//     struct OneAndGuardAndTwo {
//         owner: OneAndGuard,
//
//         #[covariant]
//         dependent: Two,
//     }
//
//     impl {Debug}
// );
//
// fn compute<'a>(one: &'a One, unrelated: &'a Unrelated) -> Two<'a> {
//     Two {
//         one,
//         // test: what if unrelated is indeed an owner?
//         unrelated,
//         // unrelated: unrelated.0.to_owned(),
//     }
// }
//
// fn build_pack(
//     one: Arc<RwLock<One>>,
//     unrelated: Arc<RwLock<Unrelated>>,
// ) -> anyhow::Result<Arc<RwLock<OneAndGuardAndTwo>>> {
//     let mut unrelated_guard: Option<RwLockReadGuard<Unrelated>> = None;
//     let one_and_guard = OneAndGuard::try_new(one, |one| {
//         // do ordered unlocking here
//         unrelated_guard = Some(unrelated.blocking_read());
//         let one_guard = one.blocking_read();
//
//         anyhow::Ok(one_guard)
//     })?;
//     let one_and_guard_and_two = OneAndGuardAndTwo::try_new(one_and_guard, |one_and_guard| {
//         anyhow::Ok(compute(
//             one_and_guard.borrow_dependent(),
//             &*unrelated_guard.unwrap(),
//         ))
//     })?;
//     Ok(Arc::new(RwLock::new(one_and_guard_and_two)))
// }
//
// fn main() -> anyhow::Result<()> {
//     let one = Arc::new(RwLock::new(One("hi_one".to_owned())));
//     let unrelated = Arc::new(RwLock::new(Unrelated("hi_unrelated".to_owned())));
//     let pack = build_pack(one.clone(), unrelated.clone())?;
//
//     // test whether it is Send and Sync
//     let mut handles = vec![];
//     for i in 0..2 {
//         let one_clone = one.clone();
//         let pack_clone = pack.clone();
//
//         handles.push(std::thread::spawn(move || {
//             let pack_clone_guard = pack_clone.blocking_read();
//             println!("[thread-{i}] one(cloned) -> {:?}", &one_clone);
//             println!("[thread-{i}] pack -> {:?}", &pack_clone_guard);
//             println!(
//                 "[thread-{i}] pack.borrow_owner() -> {:?}",
//                 pack_clone_guard.borrow_owner()
//             );
//             println!(
//                 "[thread-{i}] pack.borrow_dependent() -> {:?}",
//                 pack_clone_guard.borrow_dependent()
//             );
//         }));
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     Ok(())
// }
