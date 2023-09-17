#[no_mangle]
pub extern "C" fn hello_rust_function() {
    println!("hello_rust_function start");
    let result_of_catch_unwind = std::panic::catch_unwind(move || {
        println!("call panic");
        panic!("hi this is panic");
    });
}