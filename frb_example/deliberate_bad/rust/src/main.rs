use std::env;

fn main() {
    match &env::args().skip(1).next().unwrap()[..] {
        "RustOnly_Good" => println!("This is good code"),
        "RustOnly_StackBufferOverflow" => {
            // example from https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
            // and https://github.com/japaric/rust-san
            let xs = [0, 1, 2, 3];
            let y = unsafe { *xs.as_ptr().offset(4) };
            println!("xs={xs:?} y={y}");
        }
        "RustOnly_UseAfterFree" => {
            // https://github.com/japaric/rust-san?tab=readme-ov-file#use-after-free
            let xs = vec![0, 1, 2, 3];
            let y = xs.as_ptr();
            drop(xs);
            let z = unsafe { *y };
            println!("z={z:?}");
        }
        s => panic!("Unknown mode: {}", s),
    }
}
