mod vars;
mod ownership;
mod stack_heap;
fn main() {
    println!("Hello, world!");
    vars::run();
    ownership::run();
    stack_heap::run();
    vars::sub_a::func_a();
    vars::sub_b::func_b();
}
