import pytest
from main import process_data


def test_process_data():
    """Basic tests for the process_data function."""

    # Test with a few known inputs and expected outputs
    assert process_data([1, 2, 3]) == (0, 1)  # Product = 0, Sum = 1
    assert process_data([10, 15]) == (0, 0)  # Product = 0, Sum = 0
    assert process_data([0, 1, 2, 3, 4, 5, 6]) == (0, 8)  # Product = 0, Sum = 8
    assert process_data([100, 200, 300]) == (0, 0)  # Product = 0, Sum = 0


if __name__ == "__main__":
    pytest.main()
