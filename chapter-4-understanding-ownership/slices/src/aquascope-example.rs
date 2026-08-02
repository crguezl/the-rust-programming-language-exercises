// See https://cel.cs.brown.edu/aquascope/
// A wonderful tool to visualize ownership and borrowing in Rust.
// Run with `cargo run --bin aquascope` to see the output of this program.
fn main() {
  let mut v = vec![1, 2, 3];
  let n = &v[0]; // indexation goes first n = &(v[0])
  // same as *.index(0)
  // n borrows a reference to the first element of v, which is 1
  v.push(0);
  let x = (*n) + 1; // x = 2
  println!("x = {x}");
  println!("v = {:?}", v);
}