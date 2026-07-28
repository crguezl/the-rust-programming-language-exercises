// The second question in the Brown Rust book: https://rust-book.cs.brown.edu/ch04-04-slices.html#other-slices
// Determine whether the program will pass the compiler.
// If it passes, write the expected output of the program if it were executed.
// Run with `cargo run --bin solution`
fn main() {
    let mut s = String::from("hello");
    let bytes = s.clone().into_bytes();
    for &item in bytes.iter() {
        if item == b'l' {
            s.push_str(" world");
        }
    }
    println!(
        r#"
s            = "{s}"
&String size = {} 
&str size.   = {}"#,
        std::mem::size_of::<&String>(), // 8 bytes on a 64-bit architecture
        std::mem::size_of::<&str>(),    // 16 bytes on a 64-bit architecture
    );
}
/*
## Question 1

Consider the variables s2 and s3 in the following program. 
These two variables will be located in memory within the stack frame for main. 
Each variable has a size in memory on the stack, not including the size of pointed data. 
Which statement is true about the sizes of s2 and s3?

            fn main() {
            let s = String::from("hello");
            let s2: &String = &s;
            let s3: &str = &s[..];
            }

## Context: 

The type &String is a normal reference consisting of a single pointer,
so 8 bytes on a 64-bit architecture. The type &str is a special slice reference
which consists of a pointer and a length, so 16 bytes. Therefore s3 of type &str
uses more memory than s2 of type &String.
You can verify this yourself using std::mem::size_of, like so:

        fn main() {
        println!(
            "&String={} &str={}",
            std::mem::size_of::<&String>(),
            std::mem::size_of::<&str>(),
        );
        }
Also, note that Rust will implicitly convert string references to either &String 
or &str based on the context of the reference. So the expression &s produces 
two different values based on the expected type of &s.
*/

/*
## Question 2

Determine whether the program will pass the compiler. 
If it passes, write the expected output of the program if it were executed.

            fn main() {
            let mut s = String::from("hello");
            for &item in s.as_bytes().iter() {
                if item == b'l' {
                s.push_str(" world");
                }
            }
            println!("{s}");
            }

## Context: 

Because s.as_bytes() produces an immutable reference to s, 
it is illegal to mutate s (via push_str) inside the for-loop.

*/