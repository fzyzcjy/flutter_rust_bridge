pub struct StructWithMethod {
    pub something: String,
}

pub struct OtherStruct {
    pub u: u32,
}

impl StructWithMethod {
    pub fn new(something: String) -> StructWithMethod {
        StructWithMethod { something }
    }
    pub fn do_something(&self, _u: u32, _x: String) {
        todo!()
    }

    pub fn do_more_stuff(&self) {
        todo!()
    }

    pub fn do_huge_stuff(&self, s: String, a: OtherStruct) {
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
