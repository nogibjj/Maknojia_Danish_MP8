use std::time::Instant;

fn main() {
    // Start the timer
    let start_time = Instant::now();

    // Define input data
    let data_input: Vec<u128> = vec![0, 1, 2, 3, 4, 5, 6]; // Example input data

    // Call process_data with the input data
    let (product, total_sum) = fibonacci_performance::process_data(&data_input);

    // Measure the elapsed time
    let elapsed_time = start_time.elapsed();

    // Output the results
    println!(
        "Processed Result: Product = {}, Total Sum = {}",
        product, total_sum
    );
    println!("Running Time: {:?}", elapsed_time); // Format elapsed time
}
