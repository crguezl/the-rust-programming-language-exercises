// The second question in the Brown Rust book: https://rust-book.cs.brown.edu/ch04-04-slices.html#other-slices
// Determine whether the program will pass the compiler. 
// If it passes, write the expected output of the program if it were executed.
// Run with `cargo run --bin exercise`
fn main() {
  let mut s = String::from("hello");
  for &item in s.as_bytes().iter() {
    if item == b'l' {
      s.push_str(" world");
    }
  }
  println!("{s}");
}