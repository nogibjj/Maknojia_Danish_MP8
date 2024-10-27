// // lib.rs
// use std::fs::File;
// use std::io::{BufWriter, Write};
// use std::time::{Duration, Instant};
// use sysinfo::{System, SystemExt};

// // pub fn fibonacci(n: usize) -> Vec<u128> {
// //     let mut fib_sequence = vec![0u128, 1u128]; // Use u128 for larger Fibonacci numbers
// //     while fib_sequence.len() < n {
// //         let next =
// //             fib_sequence[fib_sequence.len() - 1].checked_add(fib_sequence[fib_sequence.len() - 2]);

// //         match next {
// //             Some(value) => fib_sequence.push(value),
// //             None => break, // Stop if adding would overflow
// //         }
// //     }
// //     fib_sequence
// // }


// pub fn process_data(data: &[u128]) -> (u128, u128) {
//     // Generate the first 10,000 Fibonacci numbers
//     let fib_data = fibonacci(10_000);
//     let filtered_data: Vec<u128> = fib_data.iter().filter(|&&x| x < 1000).copied().collect();

//     // Calculate the filtered input based on the original input
//     let filtered_input: Vec<u128> = data
//         .iter()
//         .filter(|&&x| filtered_data.contains(&x))
//         .copied()
//         .collect();

//     // Debugging statements
//     println!("Filtered Data: {:?}", filtered_data);
//     println!("Filtered Input: {:?}", filtered_input);

//     // Calculate total sum
//     let total_sum = filtered_input.iter().sum::<u128>();

//     // Debugging print statement
//     println!("Total Sum Calculation: {:?}", total_sum);

//     // Calculate product and handle zero case
//     let product = if filtered_input.is_empty() || filtered_input.contains(&0) {
//         0
//     } else {
//         // Explicitly calculate the product for debugging
//         let mut product_value = 1; // Start with 1 since it's a multiplication identity
//         for &value in &filtered_input {
//             product_value *= value;
//         }
//         product_value // Calculate product of the filtered input
//     };

//     // Debugging print statement
//     println!("Product Calculation: {:?}", product);

//     (product, total_sum) // Return both product and sum
// }

// // Other functions remain unchanged...

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fibonacci() {
//         assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
//     }

//     #[test]
//     fn test_process_data() {
//         // Test with known inputs and expected outputs
//         assert_eq!(process_data(&[1, 2, 3]), (0, 2)); // Product = 0, Sum = 2
//         assert_eq!(process_data(&[0, 1, 2, 3, 4, 5, 6]), (0, 12)); // Product = 0, Sum = 12
//     }
// }


// use std::fs::File;
// use std::io::Write;
// use std::time::Instant;
// use sysinfo::{System, SystemExt};

// Function to generate Fibonacci numbers up to the nth number
pub fn fibonacci(n: usize) -> Vec<u128> {
    let mut fib_sequence = vec![0u128, 1u128];
    while fib_sequence.len() < n {
        let next = fib_sequence[fib_sequence.len() - 1]
            .checked_add(fib_sequence[fib_sequence.len() - 2]);
        
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
    let filtered_data: Vec<u128> = fib_data.iter().filter(|&&x| x < max_value as u128).copied().collect();

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
