mod demo_ouroboros;
mod demo_self_cell;

fn main() {
    demo_ouroboros::main().unwrap();
    demo_self_cell::main().unwrap();
}
