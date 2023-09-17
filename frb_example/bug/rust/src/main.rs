fn main() {
    println!("main start");
    let _result_of_catch_unwind = std::panic::catch_unwind(move || {
        println!("call panic");
        panic!("hi this is panic");
    });
}