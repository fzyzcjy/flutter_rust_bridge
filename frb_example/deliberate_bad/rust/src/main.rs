use std::env;

fn main() {
    match &env::args().skip(1).next().unwrap()[..] {
        "good" => println!("This is a good code"),
        "stack_buffer_overflow" => {
            // example from https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
            let xs = [0, 1, 2, 3];
            let y = unsafe { *xs.as_ptr().offset(4) };
            println!("xs={xs:?} y={y}");
        }
        s => panic!("Unknown mode: {}", s),
    }
}
