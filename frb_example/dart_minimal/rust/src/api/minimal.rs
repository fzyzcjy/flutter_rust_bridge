pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub enum Hello {
    Apple,
    Orange(i32),
    Raspi {
        x: i32,
        y: i32,
    },
}

pub fn hello(a: Hello) -> Hello { a }