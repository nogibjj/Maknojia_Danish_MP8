// Function to generate Fibonacci numbers up to the nth number
pub fn fibonacci(n: usize) -> Vec<u128> {
    let mut fib_sequence = vec![0u128, 1u128];
    while fib_sequence.len() < n {
        let next =
            fib_sequence[fib_sequence.len() - 1].checked_add(fib_sequence[fib_sequence.len() - 2]);

        match next {
            Some(value) => fib_sequence.push(value),
            None => break, // Stop if adding would overflow
        }
    }
    fib_sequence
}

// Function to process data
pub fn process_data(data: &[u128]) -> (u128, u128) {
    // Determine the maximum value in the input data
    let max_value = *data.iter().max().unwrap_or(&0) as usize;

    // Generate Fibonacci numbers up to the maximum value found in the input
    let fib_data = fibonacci(max_value);
    let filtered_data: Vec<u128> = fib_data
        .iter()
        .filter(|&&x| x < max_value as u128)
        .copied()
        .collect();

    // Debugging output
    println!("Filtered Data: {:?}", filtered_data);

    // Calculate the product of the filtered Fibonacci numbers
    let mut product = 1;
    let mut has_filtered_data = false; // Flag to check if we have valid filtered data

    for number in &filtered_data {
        product *= number;
        has_filtered_data = true; // Found at least one valid number
    }

    // Set product to 0 if there are no filtered data
    if !has_filtered_data {
        product = 0;
    }

    // Calculate the sum of the filtered Fibonacci numbers
    let total_sum = filtered_data.iter().sum::<u128>();

    // Debugging print statement
    println!("Total Sum Calculation: {:?}", total_sum);
    println!("Product Calculation: {:?}", product);

    (product, total_sum) // Return both product and sum
}

// // Main function
// fn main() {
//     // Start the timer
//     let start_time = Instant::now();

//     // Define input data
//     let data_input: Vec<u128> = (1..20).collect(); // Example input data

//     // Call process_data with the input data
//     let (product, total_sum) = process_data(&data_input);

//     // Measure the elapsed time
//     let elapsed_time = start_time.elapsed();

//     // Output the results to the console
//     println!(
//         "Processed Result: Product = {}, Total Sum = {}",
//         product, total_sum
//     );
//     println!("Running Time: {:?}", elapsed_time); // Format elapsed time

//     // Generate Markdown content (you can include this part as needed)
//     // ...

//     // Print a message indicating the end of the program
//     println!("Processing complete.");
// }
