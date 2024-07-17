from typing import Any, Callable, Optional, Set, Union

def visit_collection(
    expr: Any,
    visit_fn: Union[Callable[[Any, Optional[dict]], Any], Callable[[Any], Any]],
    return_data: bool = False,
    max_depth: int = -1,
    context: Optional[dict] = None,
    remove_annotations: bool = False,
    _seen: Optional[Set[int]] = None,
) -> Any: ...
