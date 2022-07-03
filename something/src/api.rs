pub struct StructWithMethod {
    pub something: String,
}

impl StructWithMethod {
    pub fn do_something(&self, _u: u32, _x: String) {
        todo!()
    }
}

pub fn return_struct() -> StructWithMethod {
    todo!()
}

pub struct TestStruct {
    pub test_variable: String,
}

pub fn return_test_struct() -> TestStruct {
    todo!()
}
