pub fn make_stack_buffer_overflow() {
    // example from https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
    // and https://github.com/japaric/rust-san
    let xs = [0, 1, 2, 3];
    let y = unsafe { *xs.as_ptr().offset(4) };
    println!("xs={xs:?} y={y}");
}

pub fn make_heap_use_after_free() {
    // https://github.com/japaric/rust-san?tab=readme-ov-file#use-after-free
    let xs = vec![0, 1, 2, 3];
    let y = xs.as_ptr();
    drop(xs);
    let z = unsafe { *y };
    println!("z={z:?}");
}
pub fn make_use_of_uninitialized_value() {
    // https://github.com/japaric/rust-san?tab=readme-ov-file#uninitialized-read
    #[allow(deprecated, invalid_value)]
    let xs: [u8; 4] = unsafe { std::mem::uninitialized() };
    let y = xs[0] + xs[1];
    println!("y={y}");
}

pub fn make_memory_leak() {
    // https://github.com/japaric/rust-san?tab=readme-ov-file#memory-leak
    let xs = vec![0, 1, 2, 3];
    println!("xs={xs:?}");
    std::mem::forget(xs);
}

pub fn make_data_race() {
    // https://github.com/japaric/rust-san?tab=readme-ov-file#data-race
    let t1 = std::thread::spawn(|| unsafe { ANSWER = 42 });
    unsafe {
        ANSWER = 24;
    }
    t1.join().ok();
}

static mut ANSWER: i32 = 0;
