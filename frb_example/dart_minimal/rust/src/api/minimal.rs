pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub enum Hello {
    Apple,
    Orange(i32),
    Raspi {
        hello_world: i32,
        another_field: i32,
    },
}

pub fn hello(a: Hello) -> Hello {
    a
}
