// Only the first is valid, but `rust-analyzer` will accept all three
// Cargo check will reject, because `runnable_no_feat` does not use feature `feat` of crate `stuff`
fn main() {
    println!("{}", stuff::HELLO_UNCONDITIONAL);
    println!("{}", stuff::feat::HELLO_CONDITIONAL);
    println!("{}", stuff::feat::get_rand());
}
