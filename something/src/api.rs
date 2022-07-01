pub struct StructWithMethod{
    something: String
}

impl StructWithMethod {
    pub fn do_something(&self, _u: u32) {
        todo!()
    }
}

pub fn return_struct() -> StructWithMethod {
    todo!()
}