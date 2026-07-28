// https://rust-book.cs.brown.edu/ch04-04-slices.html#other-slices
fn main() {
  let mut s = String::from("hello");
  let bytes = s.clone().into_bytes();
  for &item in bytes.iter() {
    if item == b'l' {
      s.push_str(" world");
    }
  }
  println!("{s}");
}