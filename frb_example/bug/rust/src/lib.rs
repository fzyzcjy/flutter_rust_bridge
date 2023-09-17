#[no_mangle]
pub extern "C" fn hello_rust_function() {
    println!("hi backtrace {:?}", std::backtrace::Backtrace::force_capture());
    // println!("hello_rust_function start");
    // let result = std::panic::catch_unwind(move || {
    //     println!("call panic");
    //     panic!("hi this is panic");
    // });
    // println!("hello_rust_function end {:?}", result);
}