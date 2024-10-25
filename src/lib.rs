// src/lib.rs

pub fn fibonacci(n: usize) -> Vec<u128> {
    let mut fib_sequence = vec![0u128, 1u128]; // Use u128 for larger Fibonacci numbers
    while fib_sequence.len() < n {
        // Safely add the last two numbers to prevent overflow
        let next =
            fib_sequence[fib_sequence.len() - 1].checked_add(fib_sequence[fib_sequence.len() - 2]);
        match next {
            Some(value) => fib_sequence.push(value),
            None => break, // Stop if adding would overflow
        }
    }
    fib_sequence
}

pub fn process_data() -> (u128, u128) {
    // Generate Fibonacci numbers and filter for values less than 1000
    let fib_data = fibonacci(10000); // Generate the first 10,000 Fibonacci numbers
    let filtered_data: Vec<u128> = fib_data.iter().filter(|&&x| x < 1000).copied().collect();

    // Calculate the product of the filtered Fibonacci numbers
    let product = filtered_data.iter().product::<u128>();

    // Calculate the sum of the filtered Fibonacci numbers
    let total_sum = filtered_data.iter().sum::<u128>();

    (product, total_sum) // Return both product and sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_process_data() {
        // Calculate expected values
        let expected_product = 0; // Because the product includes 0
        let expected_sum = 0; // Because the sum of filtered values includes 0

        assert_eq!(process_data(), (expected_product, expected_sum));

        // You can add more tests here for additional scenarios if needed.
    }
}
