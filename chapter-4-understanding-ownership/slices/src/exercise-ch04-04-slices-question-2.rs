// https://rust-book.cs.brown.edu/ch04-04-slices.html#other-slices
fn main() {
  let mut s = String::from("hello");
  for &item in s.as_bytes().iter() {
    if item == b'l' {
      s.push_str(" world");
    }
  }
  println!("{s}");
}