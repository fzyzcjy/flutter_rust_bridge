use std::env;

fn main() {
    match &env::args().nth(1).unwrap()[..] {
        "RustOnly_Good" => println!("This is good code"),
        "RustOnly_StackBufferOverflow" => {
            // example from https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
            // and https://github.com/japaric/rust-san
            let xs = [0, 1, 2, 3];
            let y = unsafe { *xs.as_ptr().offset(4) };
            println!("xs={xs:?} y={y}");
        }
        "RustOnly_HeapUseAfterFree" => {
            // https://github.com/japaric/rust-san?tab=readme-ov-file#use-after-free
            let xs = vec![0, 1, 2, 3];
            let y = xs.as_ptr();
            drop(xs);
            let z = unsafe { *y };
            println!("z={z:?}");
        }
        "RustOnly_UseOfUninitializedValue" => {
            // https://github.com/japaric/rust-san?tab=readme-ov-file#uninitialized-read
            #[allow(deprecated, invalid_value)]
            let xs: [u8; 4] = unsafe { std::mem::uninitialized() };
            let y = xs[0] + xs[1];
            println!("y={y}");
        }
        "RustOnly_MemoryLeak" => {
            // https://github.com/japaric/rust-san?tab=readme-ov-file#memory-leak
            let xs = vec![0, 1, 2, 3];
            println!("xs={xs:?}");
            std::mem::forget(xs);
        }
        "RustOnly_DataRace" => {
            // https://github.com/japaric/rust-san?tab=readme-ov-file#data-race
            let t1 = std::thread::spawn(|| unsafe { ANSWER = 42 });
            unsafe {
                ANSWER = 24;
            }
            t1.join().ok();
        }
        s => panic!("Unknown mode: {}", s),
    }
}

static mut ANSWER: i32 = 0;
