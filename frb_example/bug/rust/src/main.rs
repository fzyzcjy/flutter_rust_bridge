fn main() {
    println!("main start");
    let result = std::panic::catch_unwind(move || {
        println!("call panic");
        panic!("hi this is panic");
    });
    println!("main end {:?}", result);
}