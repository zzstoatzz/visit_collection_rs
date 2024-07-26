import pytest
from prefect.utilities.collections import visit_collection as visit_collection_py
from visit_collection_rs import visit_collection as visit_collection_rs


@pytest.mark.parametrize(
    "test_case, visit_fn, max_depth, context",
    [
        (
            {"a": 1, "b": 2, "c": {"d": 3, "e": 4}},
            lambda x: x * 2 if isinstance(x, int) else x,
            -1,
            None,
        ),
        (
            {"x": [1, 2, {"y": 3}]},
            lambda x: str(x) if isinstance(x, int) else x,
            2,
            None,
        ),
        (
            {"a": 1, "b": 2},
            lambda x, ctx: x + ctx["add"] if isinstance(x, int) else x,
            -1,
            {"add": 10},
        ),
        ({1: "a", 2: "b"}, lambda x: x * 2 if isinstance(x, int) else x, 0, None),
    ],
    ids=["nested_dict", "nested_list", "context_usage", "zero_depth"],
)
def test_visit_dict(test_case, visit_fn, max_depth, context):
    py_result = visit_collection_py(
        test_case, visit_fn, return_data=True, max_depth=max_depth, context=context
    )
    rs_result = visit_collection_rs(
        test_case, visit_fn, return_data=True, max_depth=max_depth, context=context
    )

    assert rs_result == py_result, f"Mismatch: Python: {py_result}, Rust: {rs_result}"
