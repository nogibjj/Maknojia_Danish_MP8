## Maknojia_Danish_MP8

[![CI](https://github.com/nogibjj/Maknojia_Danish_MP8/actions/workflows/pythoncicd.yml/badge.svg)](https://github.com/nogibjj/Maknojia_Danish_MP8/actions/workflows/pythoncicd.yml)

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
