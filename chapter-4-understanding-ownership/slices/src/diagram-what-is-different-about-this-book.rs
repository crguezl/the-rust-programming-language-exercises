// See section https://rust-book.cs.brown.edu/experiment-intro.html#content-changes inside the chapter
// "What’s Different About This Book?"
// Run with `cargo run --bin diagram` to see the output of this program.
fn main() {
    let mut s = String::from("Hello world");
    println!("s = {s}");
    let hello = &s[0..5]; // an inmutable borrow of s
    println!("hello = {hello}");
    s.push_str("!"); // Line M
    println!("s = {s}");
    // println!("hello = {hello}"); // Error: hello is inmutable and the line M mutates s, so hello is no longer valid after line M
   drop(s);
}
