import time
import psutil


def fibonacci(n):
    """Generate Fibonacci numbers up to the nth number."""
    fib_sequence = [0, 1]
    while len(fib_sequence) < n:
        fib_sequence.append(fib_sequence[-1] + fib_sequence[-2])
    return fib_sequence


def process_data(data):
    # Generate Fibonacci numbers and filter for values less than 1000
    fib_data = fibonacci(max(data))
    filtered_data = [x for x in fib_data if x < 1000]

    # Calculate the product of the filtered Fibonacci numbers
    product = 1
    for number in filtered_data:
        product *= number

    return product


def get_memory_usage():
    process = psutil.Process()
    mem_info = process.memory_info()
    return mem_info.rss / 1024**2  # Convert bytes to MB


# Measure the running time and memory usage
def measure_performance(x):
    initial_memory = get_memory_usage()
    start_time = time.time()
    result = process_data(x)
    end_time = time.time()
    final_memory = get_memory_usage()

    # Calculate time and memory used
    runtime = end_time - start_time
    memory_used = final_memory - initial_memory

    print(f"Processed Result: {result}")
    print(f"Running Time: {runtime:.6f} seconds")
    print(f"Memory Usage: {memory_used:.6f} MB")


if __name__ == "__main__":
    data_input = list(range(1, 20))  # Adjusted range for Fibonacci numbers
    measure_performance(data_input)
