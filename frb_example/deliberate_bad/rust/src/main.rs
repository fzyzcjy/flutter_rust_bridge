use frb_example_deliberate_bad::api::simple::*;
use std::env;

fn main() {
    match &env::args().skip(1).next().unwrap()[..] {
        "RustOnly_Good" => println!("This is good code"),
        "RustOnly_StackBufferOverflow" => make_stack_buffer_overflow(),
        "RustOnly_HeapUseAfterFree" => make_heap_use_after_free(),
        s => panic!("Unknown mode: {}", s),
    }
}
