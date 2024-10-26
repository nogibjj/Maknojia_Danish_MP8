import pytest
from main import process_data


def test_process_data():
    """Basic tests for the process_data function."""

    # Test with a few known inputs and expected outputs
    assert process_data([1, 2, 3]) == (0, 2)  # Product = 0, Sum = 2
    assert process_data([0, 1, 2, 3, 4, 5, 6]) == (0, 12)  # Product = 0, Sum =12


if __name__ == "__main__":
    pytest.main()
