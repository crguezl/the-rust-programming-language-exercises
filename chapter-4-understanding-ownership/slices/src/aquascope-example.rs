// See https://cel.cs.brown.edu/aquascope/
// A wonderful tool to visualize ownership and borrowing in Rust.
// Run with `cargo run --bin aquascope` to see the output of this program.
fn main() {
  let mut v = vec![1, 2, 3];
  let n = &v[0]; // indexation goes first n = &(v[0])
  // same as *.index(0)
  // n borrows a reference to the first element of v, which is 1
  v.push(0); // It can produce UB: a reallocation of the vector, which invalidates the reference n.
  v[2] = 5; // v is mutated, but n is still valid because it is a reference to the first element of v
  let x = (*n) + 1;   //  Move these two lines before the mutation of v  and the error will disappear. 
  println!("x = {x}"); // The reason is that the borrow checker will see that n is no longer used after the mutation of v, so it will allow the mutation. If we move the lines after the mutation of v, the borrow checker will see that n is still used after the mutation, so it will not allow the mutation.

  println!("v = {:?}", v);
}