pub fn fibonacci(n: usize) -> Vec<u128> {
    let mut fib_sequence = vec![0u128, 1u128]; // Use u128 for larger Fibonacci numbers
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

pub fn process_data(data: &[u128]) -> (u128, u128) {
    // Filter Fibonacci numbers generated for values less than 1000
    let fib_data = fibonacci(10_000); // Generate the first 10,000 Fibonacci numbers
    let filtered_data: Vec<u128> = fib_data.iter().filter(|&&x| x < 1000).copied().collect();

    // Calculate the product and sum of filtered data based on the input
    let filtered_input: Vec<u128> = data
        .iter()
        .filter(|&&x| filtered_data.contains(&x))
        .copied()
        .collect();

    // Calculate the product of the filtered input values
    let product = filtered_input.iter().product::<u128>();

    // Calculate the sum of the filtered input values
    let total_sum = filtered_input.iter().sum::<u128>();

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
        // Test with known inputs and expected outputs
        assert_eq!(process_data(&[1, 2, 3]), (0, 2)); // Product = 0, Sum = 2
        assert_eq!(process_data(&[0, 1, 2, 3, 4, 5, 6]), (0, 12)); // Product = 0, Sum = 12
    }
}
