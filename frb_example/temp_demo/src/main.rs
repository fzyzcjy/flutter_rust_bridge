mod demo_ouroboros;
mod demo_self_cell;
mod user_code;
mod demo_mine;

fn main() {
    demo_ouroboros::main().unwrap();
    demo_self_cell::main().unwrap();
}
