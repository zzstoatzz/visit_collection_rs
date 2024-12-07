from prefect.utilities.collections import visit_collection as visit_collection_py
from visit_collection_rs import visit_collection


def add_one(x):
    if isinstance(x, int):
        return x + 1
    return x


def test_list() -> bool:
    print("testing list")
    some_list = [1, 2, 3]
    result = visit_collection(some_list, add_one, return_data=True)
    result_py = visit_collection_py(some_list, add_one, return_data=True)
    assert result == [2, 3, 4], f"Expected [2, 3, 4], got {result}"
    assert result == result_py, f"Expected {result_py}, got {result}"
    print(f" result_rs: {result}")
    print(f" result_py: {result_py}")
    return True


def test_dict() -> bool:
    print("testing dict")
    some_dict = {"a": 1, "b": 2}
    result = visit_collection(some_dict, add_one, return_data=True)
    result_py = visit_collection_py(some_dict, add_one, return_data=True)
    assert result == {"a": 2, "b": 3}, f"Expected {{'a': 2, 'b': 3}}, got {result}"
    assert result == result_py, f"Expected {result_py}, got {result}"
    print(f" result_rs: {result}")
    print(f" result_py: {result_py}")
    return True


if __name__ == "__main__":
    assert all([test_list(), test_dict()])
