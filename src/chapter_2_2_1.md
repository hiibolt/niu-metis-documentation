# 2.2.1. A Parallel Rust Project, from the Ground Up
*You can find the code mentioned in this chapter [in this book's repository](https://github.com/hiibolt/niu-metis-documentation/tree/main/projects/rust/basic-rayon)!*

Rust is a massive language with many intricate types that can be used to facilitate the representation of almost anything imagineable.

However, we'll be focusing on only a few, namely due to limitations imposed by the MPI standard - we can't trivially pass data that isn't supported by their API.

These types are laid out in the documentation for the `mpi` crate [here](https://docs.rs/mpi/latest/mpi/datatype/trait.Equivalence.html), but to summarize:
- **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `isize`
- **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `usize`
- **Floating-Point Numbers**: `f32`, `f64`
- **Booleans**: `bool`

Don't overthink it - you probably want `f64` or `i32`/`i64` in nearly every case. Use `usize` for indexing. If you come from C++, you can find a nice guide [here](https://cel.cs.brown.edu/crp/idioms/type_equivalents.html) that shows what's what!

I'll run you through the basics, but I won't keep you long - there are plenty of [great guides](https://www.rust-lang.org/learn) that do a better job teaching it than I ever could.

First, variables are declared with the following syntax:
```
let [mut] var: <type> = <expression>;
```

Variables are **immutable** by default. Here's an example where we create a (mutable) vector of `f64`s:
```rust
let mut nums: Vec<f64> = Vec::new();
```

Rust also has arguably the best macro support out of any language, one such example being the `vec!(<num>; <len>)` macro that fills an array out:
```rust
let mut nums: Vec<f64> = vec!(0.0f64; 100_000_000);
```

Rereading above, you can see that the declaration expects an **expression**. Almost everything is an expression, including **block expressions**, meaning the following is completely valid Rust code:
```rust
let mut nums: Vec<f64> = {
    let mut inner_nums = Vec::new();
    for _ in 0..100_000_000 {
        inner_nums.push(0);
    }
    inner_nums
};
```
...where `_` is a wildcard "forget-about-it" operator, since we don't care about the index here.

Notice the implied return at the end of the block - we could equivalently write ```return inner_nums;```, but it's more "Rust-y" to do it this way!

### Getting Started
Let's get started with an example. Metis comes with `cargo` installed, which is the everything-bagel for development in Rust.

First, create a new project:
```bash
$ mkdir -p ~/projects/rust/basic-rayon
$ cd ~/projects/rust/basic-rayon
$ cargo init .
$ cargo run
```

You'll be met with a kind `Hello, World!`. Nice work, you're basically a Rust developer now.

All Rust source code is in the `src` directory, with the entrypoint being `main.rs`. Let's rewrite this to start strong with a program that generates a whole bunch of random floats!

### Creating a `Vec` of Random Numbers
First, we'll need the `rand` crate (a Rust library) to make random numbers. We'll also go ahead and add the `rayon` crate for later:
```bash
$ cargo add rand
$ cargo add rayon
```

Let's now write our main function!
```rust
use rand::prelude::*;

fn main() {
    let mut nums: Vec<f64> = vec!(0.0; 1_000_000);
    rand::rng().fill(&mut nums[..]);
    
    println!("{nums:?}");
}
```

Run with `cargo run`, and you'll be met with a slew of output. Nice work! We'll sorta black box what exactly is happening with `rand`, but to summarize:
```rust
use rand::prelude::*;                              // Import `rand`

fn main() {                                        
    let mut nums: Vec<f64> = vec!(0.0; 1_000_000); // Make a zero vector
    rand::rng().fill(&mut nums[..]);               // Fill it with random floats

    println!("{nums:?}");                          
}
```

Notice that we're using the `println!` macro - we know it's a macro because of the bang (`!`) - to format the `nums` vector into its debugging representation with `:?`.

This is required because unlike a primitive type like `f64`, there's no standard string representation of a vector:
```
[0, 1, 2, 3] // like this?
0, 1, 2, 3   // maybe this?
0 1 2 3      // what about this?
```

### Sequentially Calculating the Average
Well, alright, we've gone ahead and created our vector of random `f64`s. How do we calculate the average?

Functions in Rust use the following syntax:
```
fn <name> ( [arg1: <type>], [arg2: <type>], ... ) [-> <type>] {
    ...
}
```

Arguments are optional, but must have specified types. The return type for a function is assumed to be `()`, an empty tuple.

This can somewhat be thought of as `void` from C++, but it's not the same - `()` is a Zero-Sized Type that occupies absolutely no memory. If that sounds insane, I thought so too, but it's legit - the [docs on it](https://doc.rust-lang.org/nomicon/exotic-sizes.html) are pretty interesting.

Jumping back to function syntax, here's a rudimentary example of an `add` function:
```rust
fn add ( num_1: f64, num_2: f64 ) -> f64 {
    num_1 + num_2
}
```
...notice again the implicit return, Rust's neat like that!

So to write our average function, we're going to take advantage of Rust's absolutely phenomenal [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) type, which has an outrageous number of useful methods. We'll write a function which uses two for our average:
```rust
fn sequential_average ( nums: &[f64] ) -> f64 {
    nums.into_iter().sum::<f64>() / (nums.len() as f64)
}
```

Let's break this down.
* 1.) `&[f64]` is a reference to an array of `f64`s
* 2.) `.into_iter()` converts our array reference into an `Iterator<f64>`
* 3.) `.sum()` summates our `Iterator<f64>` into one `f64`
* 4.) `.len()` returns a `usize`, which we can cast to an f64

And this works! If we append it to our main function (consider removing the `println!(...)`):
```rust
fn main ( ) {
    ...

    // Test the sequential average
    let start = Instant::now();
    let avg = sequential_average(&nums);
    let duration = start.elapsed();
    println!("Average: {} in {:?}", avg, duration);

    ...
}
```

When testing with a much larger number, it's actually pretty quick, it gets the job done for 100,000,000 elements in about ~900ms on Metis! 

Very quick - Rust is fast - but we can be faster. Let's talk about `rayon`.

### Parallel Computation with `rayon`
To get started with `rayon`, let's import its prelude and something to help us measure:
```rust
use rayon::prelude::*;
use std::time::Instant;
```

Now, what's unique about `rayon` in our case is the [`ParallelSlice`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSlice.html) that it offers. It allows you to effectively convert our regular slice into a slice that can be easily operated on in parallel.

I'm gonna throw this at you all at once, but I'll break it down after, I promise!
```rust
fn parallel_average ( nums: &[f64] ) -> f64 {
    let sum: f64 = nums
        .par_chunks(nums.len() / 100)
        .map(|chunk| {
            chunk.into_iter()
                .map(|&n| n)
                .sum::<f64>()
        })
        .sum();

    sum / (nums.len() as f64)
}
```

Our function has the same inputs and outputs as its sequential equivalent. Similarly, we want to start by creating an iterator. 

Instead of using `.into_iter()`, which creates an iterator of elements); we'll use `.par_chunks(<n>)`, which creates an iterator of chunks which are `n` length big.

Next, we use `.map(<closure>)`, which takes a closure that should accept a chunk and return its sum. For the sake of brevity, assume that a closure is a lambda function from C++, they're pretty similar here.

Just like the previous sequential implementation, we create an iterator and summate it. The sole difference is that since `.par_chunks` creates chunks that are `&[&f64]` instead of `&[f64]`, we dereference them to their actual values with `.map(|&n| n)`.

Lastly, we divide the sum by the length to create our average. Let's add testing code to our `main` function:
```rust
fn main ( ) {
    ...

    // Test the sequential average
    let start = Instant::now();
    let avg = sequential_average(&nums);
    let duration = start.elapsed();
    println!("Average: {} in {:?}", avg, duration);

    ...
}
```

All together, our program should come out as the following:
```rust
use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // Create a vector of n random integers
    let mut nums: Vec<f64> = vec!(0.0; 100_000_000);
    rand::rng().fill(&mut nums[..]);

    // Test the parallel average
    let start = Instant::now();
    let avg = parallel_average(&nums);
    let duration = start.elapsed();
    println!("Average: {} in {:?}", avg, duration);

    // Test the sequential average
    let start = Instant::now();
    let avg = sequential_average(&nums);
    let duration = start.elapsed();
    println!("Average: {} in {:?}", avg, duration);
}

fn sequential_average ( nums: &[f64] ) -> f64 {
    nums.into_iter().sum::<f64>() / nums.len() as f64
}
fn parallel_average ( nums: &[f64] ) -> f64 {
    let sum: f64 = nums
        .par_chunks(nums.len() / 100)
        .map(|chunk| {
            chunk.into_iter()
                .map(|&n| n)
                .sum::<f64>()
        })
        .sum();

    sum / (nums.len() as f64)
}
```

Lastly, let's compile and not in the debug profile, as `cargo run` does, but in release mode:
```bash
$ cargo build --release
   Compiling rayon-test v0.1.0 (/nfs/ihfs/home_metis/z1994244/projects/rust/rayon-test)
    Finished `release` profile [optimized] target(s) in 0.34s
$ ./target/release/rayon-test
Average: 0.5000479638601427 in 21.319388ms
Average: 0.5000479638602693 in 126.604707ms
```

Nicely done! And wow - sure makes a difference, doesn't it?