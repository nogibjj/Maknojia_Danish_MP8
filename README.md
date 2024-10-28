## Maknojia_Danish_MP8

[![CI](https://github.com/nogibjj/Maknojia_Danish_MP8/actions/workflows/pythoncicd.yml/badge.svg)](https://github.com/nogibjj/Maknojia_Danish_MP8/actions/workflows/pythoncicd.yml)


# Analysis of the Python Fibonacci Data Processing Code

## Overview
This Python & Rust code aims to process a series of Fibonacci numbers based on input data and measures the program's performance in terms of execution time and memory usage. It generates Fibonacci numbers, filters them, calculates their product and sum, and outputs the performance data in a Markdown file.

## Performance Considerations
- **Memory Usage**: The program captures memory usage before and after data processing to calculate the memory overhead of generating and filtering the Fibonacci sequence.
- **Execution Time**: Measures the time taken to process the input data, which can vary based on the size of the input list and the maximum value used for generating the Fibonacci sequence.
- **Input Data Flexibility**: The `process_data` function's approach of generating Fibonacci numbers up to the maximum value in the input ensures that the sequence is only as large as necessary.

## Conclusion
The code is efficient for generating and analyzing Fibonacci sequences up to a specified range and provides insights into its memory and time efficiency. It is suitable for small-scale data processing tasks, while the markdown generation adds value by documenting the performance analysis for future reference.


# Performance Comparison: Rust vs. Python for Fibonacci Data Processing

## Input Data
```text
[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]

```

## Comparison

| Metric           | Rust                          | Python                        |
|------------------|-------------------------------|-------------------------------|
| **Memory Usage** | 868,352 bytes (~847 KB)       | 16,384 bytes (~16 KB)         |
| **Runtime**      | 105,375 µs (~0.105 s)         | 41 µs (~0.000041 s)           |
| **Results**      | Product = 0, Sum = 33         | Product = 0, Sum = 33         |

- **Memory Efficiency**: Python uses ~98% less memory, making it more suitable for small-scale tasks.
- **Execution Speed**: Python is faster due to lower overhead, while Rust's extra time comes from system-level operations.
- **Use Case**: Python is ideal for small tasks, while Rust shines in larger, more complex, or resource-heavy scenarios.
