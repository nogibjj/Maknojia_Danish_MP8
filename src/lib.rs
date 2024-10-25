pub fn fibonacci(n: usize) -> Vec<u128> {
    let mut fib_sequence = vec![0u128, 1u128]; // Use u128 for larger Fibonacci numbers
    while fib_sequence.len() < n {
        let next = fib_sequence[fib_sequence.len() - 1].checked_add(fib_sequence[fib_sequence.len() - 2]);
        match next {
            Some(value) => fib_sequence.push(value),
            None => break, // Stop if adding would overflow
        }
    }
    fib_sequence
}

pub fn process_data(data: &[usize]) -> u128 {
    // Generate Fibonacci numbers and filter for values less than 1000
    let max_value = *data.iter().max().unwrap_or(&0);
    let fib_data = fibonacci(max_value);
    let filtered_data: Vec<u128> = fib_data.iter().filter(|&&x| x < 1000).copied().collect();

    // Calculate the product of the filtered Fibonacci numbers
    let product = filtered_data.iter().product();

    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        assert_eq!(process_data(&[1, 5, 10, 12, 8, 20]), 0); // Product includes 0
        assert_eq!(process_data(&[]), 0); // Empty input
        assert_eq!(process_data(&[32, 1, 2]), 0); // Product includes 0
        assert_eq!(process_data(&[3, 5, 7, 9]), 0); // Product includes 0
        assert_eq!(process_data(&[0, 1, 2, 3, 4, 5, 6]), 0); // Product includes 0
        assert_eq!(process_data(&[21, 34, 55]), 0); // Product includes 0
        assert_eq!(process_data(&[100, 200, 300]), 0); // No valid Fibonacci numbers; product = 0
        assert_eq!(process_data(&[1000, 2000]), 0); // No valid Fibonacci numbers; product = 0
        assert_eq!(process_data(&(1..30).collect::<Vec<_>>()), 0); // Up to 30; should still produce 0
    }
}
