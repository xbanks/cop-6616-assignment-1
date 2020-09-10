# Parallel Prime Finding

## Approach

The main approach I used here is a modified version of the Sieve of Eratosthenes. The main modification made was to first create the initial range of `sqrt(10^8)` numbers, filter out the even numbers, shuffle them, and pass each of those ranges to a separate thread which would then check off which numbers were not prime. Each thread then passed back it's completed set of non-primes, that we were then able to use to filter the initial list down to the actual primes.

The shuffling is to ensure that each thread ends up with an even amount of work.

## Correctness

This should be correct because it performs essentially the same process as the original sieve. 

## Running

This is a rust project, so after rust is installed on your system via the [Rust Website](https://www.rust-lang.org/tools/install), all you need to do is run `cargo run --release` to compile and run the release version of this application. It will then write an `output.txt` file to the current directory.
