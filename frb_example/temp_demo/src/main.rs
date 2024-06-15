use ouroboros::self_referencing;
use std::sync::{Arc, RwLock, RwLockReadGuard};

#[self_referencing]
struct MyStruct {
    upstream_arg: Arc<RwLock<String>>,
    #[borrows(upstream_arg)]
    read_guard: RwLockReadGuard<'this, String>,
}

fn main() {
    // let mut my_value = MyStructBuilder {
    //     int_data: 42,
    //     int_reference_builder: |int_data: &i32| int_data,
    // }
    // .build();
    //
    // println!("{:?}", my_value.borrow_int_data());
}
