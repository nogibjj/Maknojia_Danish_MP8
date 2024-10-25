from main import process_data


def test_process_data():
    """Testing the process_data function with Fibonacci numbers."""

    # Test with a range that includes several Fibonacci numbers
    assert (
        process_data([1, 5, 10, 12, 8, 20]) == 0
    )  # All Fibonacci numbers less than 1000 are: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987. Product = 0
    assert process_data([1, 2, 3]) == 0  # Only 0 and 1 are valid; product is 0
    assert (
        process_data([10, 15]) == 0
    )  # Contains no Fibonacci numbers less than 1000; product is 0
    assert (
        process_data([3, 5, 7, 9]) == 0
    )  # Fibonacci numbers < 1000 are: 0, 1, 1, 2, 3, 5. Product = 0
    assert (
        process_data([0, 1, 2, 3, 4, 5, 6]) == 0
    )  # Only 0, 1, 1, 2, 3, 5; product is 0
    assert (
        process_data([8, 13, 21]) == 0
    )  # Contains valid Fibonacci numbers, but product includes 0
    assert process_data([1, 2, 8]) == 0  # Only Fibonacci numbers are valid; product = 0
    assert (
        process_data([21, 34, 55]) == 0
    )  # Product of Fibonacci numbers under 1000 = 0

    # Adding cases with no Fibonacci numbers < 1000
    assert process_data([100, 200, 300]) == 0  # No valid Fibonacci numbers; product = 0
    assert process_data([1000, 2000]) == 0  # No valid Fibonacci numbers; product = 0

    # Check with a larger input range
    assert (
        process_data(list(range(1, 30))) == 0
    )  # Up to 30; should still produce 0 as there is 0 in the input


if __name__ == "__main__":
    test_process_data()
    print("All tests passed!")
