#[derive(Debug)]
pub struct One(pub String);

#[derive(Debug)]
pub struct Unrelated(pub String);

#[derive(Debug)]
pub struct Two<'a> {
    pub one: &'a One,
    // test: what if unrelated is indeed an owner?
    // unrelated: &'a Unrelated,
    pub unrelated: String,
}

