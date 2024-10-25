use std::time::Instant;
use fibonacci_performance::{process_data};

fn main() {
    let data_input = (1..20).collect::<Vec<_>>(); // Create a vector of integers from 1 to 19
    let start_time = Instant::now();
    let result = process_data(&data_input);
    let elapsed_time = start_time.elapsed();

    println!("Processed Result: {}", result);
    println!("Running Time: {:?}", elapsed_time);
}
