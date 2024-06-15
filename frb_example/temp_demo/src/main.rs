use ouroboros::self_referencing;

#[self_referencing]
struct MyStruct {
    int_data: i32,
    #[borrows(int_data)]
    int_reference: &'this i32,
}

fn main() {
    let mut my_value = MyStructBuilder {
        int_data: 42,
        int_reference_builder: |int_data: &i32| int_data,
    }
    .build();

    println!("{:?}", my_value.borrow_int_data());
}
