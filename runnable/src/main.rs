// All three prints should be possible, because `runnable` uses feature `feat` of crate `stuff`
fn main() {
    println!("{}", stuff::HELLO_UNCONDITIONAL);
    println!("{}", stuff::feat::HELLO_CONDITIONAL);
    println!("{}", stuff::feat::get_rand());
}
