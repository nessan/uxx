// This example demonstrates how to iterate through all u52s with a fixed number of bits set (n_set) in increasing
// order, using the u52_for_index and index_for_u52 functions. It also includes assertions to verify the properties of
// the generated u52s and their corresponding indices. The progress is indicated by printing a dot every n_tick
// iterations.
//
// This should be run with optimizations enabled (e.g., `cargo run --release`) to complete in a reasonable time frame,
// as it involves a large number of iterations.

use std::io::Write;
use uxx::*;

fn main() {
    let n_set = 13;

    let n_trials = 10_000_000_u64;
    let n_tick = n_trials / 40;

    // The first index corresponds to the smallest u52 with 13 bits set, which is 0b1111111111111 (13 ones).
    let prev = u52_for_index(0, n_set);
    assert_eq!(0b1111111111111, prev);
    assert_eq!(0, index_for_u52(prev));

    // The later indices correspond to ever increasing u52s with 13 bits set.
    println!("Iterating through the first {} u52s with {} set bits:", n_trials, n_set);
    for n in 1..n_trials {
        let next = u52_for_index(n, n_set);

        // Check some properties of the next u52.
        assert!(prev < next);
        assert_eq!(n, index_for_u52(next));
        assert_eq!(n_set, next.count_ones());

        // Occasionally add a tick to the progress "bar".
        if n % n_tick == 0 {
            print!(".");
            std::io::stdout().flush().unwrap();
        }
    }
    println!("\nAll tests passed!");
}
