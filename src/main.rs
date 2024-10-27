use std::fs::File;
use std::io::Write;
use std::time::Instant;
use systemstat::{Platform, System};

fn main() {
    // Start the timer
    let start_time = Instant::now();

    // Define input data
    let data_input: Vec<u128> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]; // Example input data

    // Create a System instance to fetch memory statistics
    let system = System::new();

    // Measure memory usage before processing
    let initial_memory =
        system.memory().unwrap().total.as_u64() - system.memory().unwrap().free.as_u64(); // Calculate used memory

    // Call process_data with the input data
    let (product, total_sum) = fibonacci_performance::process_data(&data_input);

    // Measure the elapsed time
    let elapsed_time = start_time.elapsed();

    // Measure memory usage after processing
    let final_memory =
        system.memory().unwrap().total.as_u64() - system.memory().unwrap().free.as_u64(); // Calculate used memory

    // Calculate memory used during processing
    let memory_used = (final_memory - initial_memory) / 1024; // Convert bytes to KB

    // Output the results to the console
    println!(
        "Processed Result: Product = {}, Total Sum = {}",
        product, total_sum
    );
    println!("Running Time: {:?}", elapsed_time); // Format elapsed time
    println!("Memory Usage: {} KB", memory_used);

    // Generate Markdown content
    let markdown_content = format!(
        "# Performance Analysis of Fibonacci Data Processing\n\n\
        ## Overview\n\n\
        This document details the performance analysis of a Rust program that processes Fibonacci numbers. \
        The program computes a product and sum of filtered Fibonacci numbers for a given input set and \
        measures the execution time and memory usage.\n\n\
        ## Input Data\n\n\
        The input data used for the test is a vector of unsigned 128-bit integers:\n\n\
        ```rust\n\
        let data_input: Vec<u128> = {:?};\n\
        memory_usage = {};\n\
        runtime = {:?};\n\
        results = ({}, {});\n\
        ```\n",
        data_input, memory_used, elapsed_time.as_micros(), product, total_sum
    );

    // Write to a Markdown file
    let mut file = File::create("performance_analysis.md").expect("Unable to create file");
    file.write_all(markdown_content.as_bytes())
        .expect("Unable to write data");

    println!("Markdown file 'performance_analysis.md' has been generated.");
}
