import pytest
from prefect.utilities.collections import visit_collection as visit_collection_py
from visit_collection_rs import visit_collection as visit_collection_rs


def double_rations(x):
    return x * 2 if isinstance(x, int) else x


def share_feast(x, ctx):
    if isinstance(x, int):
        return x + ctx["portions"]
    return [i + ctx["portions"] for i in x]


@pytest.mark.parametrize(
    "test_list,visit_fn,max_depth,context",
    [
        (
            [[1, 2], [3, 4], [5, 6]],
            lambda x: x * 2 if isinstance(x, int) else x,
            -1,
            None,
        ),
        ([1, [2, [3, [4]]]], double_rations, 2, None),
        (
            [1, 2, 3],
            share_feast,
            -1,
            {"portions": 10},
        ),
    ],
    ids=["nested_list", "max_depth", "context_usage"],
)
def test_visit_collection(test_list, visit_fn, max_depth, context):
    py_result = visit_collection_py(
        test_list, visit_fn, return_data=True, max_depth=max_depth, context=context
    )
    rs_result = visit_collection_rs(
        test_list, visit_fn, return_data=True, max_depth=max_depth, context=context
    )

    assert (
        rs_result == py_result
    ), f"Rust implementation doesn't match Python. Python: {py_result}, Rust: {rs_result}"

    py_result_no_return = visit_collection_py(
        test_list, visit_fn, return_data=False, max_depth=max_depth, context=context
    )
    rs_result_no_return = visit_collection_rs(
        test_list, visit_fn, return_data=False, max_depth=max_depth, context=context
    )

    assert (
        rs_result_no_return == py_result_no_return
    ), "Rust implementation doesn't match Python for return_data=False"
