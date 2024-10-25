use std::time::Instant;

fn main() {
    // Start the timer
    let start_time = Instant::now();

    // Call process_data without passing any arguments
    let (product, total_sum) = fibonacci_performance::process_data();

    // Measure the elapsed time
    let elapsed_time = start_time.elapsed();

    // Output the results
    println!(
        "Processed Result: Product = {}, Total Sum = {}",
        product, total_sum
    );
    println!("Running Time: {:.6?} seconds", elapsed_time); // Format elapsed time
}
