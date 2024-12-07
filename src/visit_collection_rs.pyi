from typing import Any, Callable

def visit_collection(
    expr: Any,
    visit_fn: Callable[[Any, dict[Any, Any] | None], Any] | Callable[[Any], Any],
    return_data: bool = False,
    max_depth: int = -1,
    context: dict[Any, Any] | None = None,
    remove_annotations: bool = False,
    _seen: set[int] | None = None,
) -> Any: ...
