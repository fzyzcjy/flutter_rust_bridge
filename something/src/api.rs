pub struct StructWithMethod{
    pub something: String
}

impl StructWithMethod {
    pub fn do_something(&self, _u: u32, _x: String) {
        todo!()
    }
}

pub fn return_struct() -> StructWithMethod {
    todo!()
}