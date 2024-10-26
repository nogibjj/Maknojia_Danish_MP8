#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data() {
        // Test with known inputs and expected outputs
        assert_eq!(process_data(&[1, 2, 3]), (0, 2)); // Product = 0, Sum = 2
        assert_eq!(process_data(&[0, 1, 2, 3, 4, 5, 6]), (0, 12)); // Product = 0, Sum = 12

}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_process_data() {
//         assert_eq!(process_data(&[1, 5, 10, 12, 8, 20]), 0); // Product includes 0
//         assert_eq!(process_data(&[]), 0); // Empty input
//         assert_eq!(process_data(&[32, 1, 2]), 0); // Product includes 0
//         assert_eq!(process_data(&[3, 5, 7, 9]), 0); // Product includes 0
//         assert_eq!(process_data(&[0, 1, 2, 3, 4, 5, 6]), 0); // Product includes 0
//         assert_eq!(process_data(&[21, 34, 55]), 0); // Product includes 0
//         assert_eq!(process_data(&[100, 200, 300]), 0); // No valid Fibonacci numbers; product = 0
//         assert_eq!(process_data(&[1000, 2000]), 0); // No valid Fibonacci numbers; product = 0
//         assert_eq!(process_data(&(1..30).collect::<Vec<_>>()), 0); // Up to 30; should still produce 0
//     }
// }
