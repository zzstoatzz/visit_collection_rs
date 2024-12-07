import pytest
from prefect.utilities.collections import visit_collection as visit_collection_py
from visit_collection_rs import visit_collection as visit_collection_rs


def uppercase_strings(x):
    return x.upper() if isinstance(x, str) else x


def add_context(x, ctx):
    if isinstance(x, str):
        return x + ctx["suffix"]
    return x


@pytest.mark.parametrize(
    "test_dict,visit_fn,max_depth,context",
    [
        (
            {"a": 1, "b": 2},
            uppercase_strings,
            -1,
            None,
        ),
        (
            {"a": {"b": {"c": "hello"}}},
            uppercase_strings,
            2,
            None,
        ),
        (
            {"key": "value", "nested": {"key": "value"}},
            add_context,
            -1,
            {"suffix": "_modified"},
        ),
    ],
    ids=["simple_dict", "max_depth", "context_usage"],
)
def test_visit_dict(test_dict, visit_fn, max_depth, context):
    py_result = visit_collection_py(
        test_dict, visit_fn, return_data=True, max_depth=max_depth, context=context
    )
    rs_result = visit_collection_rs(
        test_dict, visit_fn, return_data=True, max_depth=max_depth, context=context
    )

    assert (
        rs_result == py_result
    ), f"Rust implementation doesn't match Python. Python: {py_result}, Rust: {rs_result}"

    py_result_no_return = visit_collection_py(
        test_dict, visit_fn, return_data=False, max_depth=max_depth, context=context
    )
    rs_result_no_return = visit_collection_rs(
        test_dict, visit_fn, return_data=False, max_depth=max_depth, context=context
    )

    assert (
        rs_result_no_return == py_result_no_return
    ), "Rust implementation doesn't match Python for return_data=False"
