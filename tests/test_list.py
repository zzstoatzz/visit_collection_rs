import pytest
from prefect.utilities.collections import visit_collection as visit_collection_py
from visit_collection_rs import visit_collection as visit_collection_rs


@pytest.mark.parametrize(
    "test_list,visit_fn",
    [
        ([[1, 2], [3, 4], [5, 6]], lambda x: x * 2 if isinstance(x, int) else x),
    ],
)
def test_visit_collection(test_list, visit_fn):
    py_result = visit_collection_py(test_list, visit_fn, return_data=True)
    rs_result = visit_collection_rs(test_list, visit_fn, return_data=True)

    assert (
        rs_result == py_result == [[2, 4], [6, 8], [10, 12]]
    ), f"Rust implementation doesn't match Python. Python: {py_result}, Rust: {rs_result}"

    py_result_no_return = visit_collection_py(test_list, visit_fn, return_data=False)
    rs_result_no_return = visit_collection_rs(test_list, visit_fn, return_data=False)

    assert (
        rs_result_no_return == py_result_no_return
    ), "Rust implementation doesn't match Python for return_data=False"
