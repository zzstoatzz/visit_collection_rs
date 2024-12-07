use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PySet};

mod dict;
mod list;

// Only copy context if needed, and try to reuse where possible
pub(crate) fn maybe_copy_context<'py>(
    py: Python<'py>,
    context: Option<&PyDict>,
) -> PyResult<Option<&'py PyDict>> {
    if let Some(ctx) = context {
        // If context might be modified by visit_fn and must not leak upward
        // we must copy. If not, consider returning the same reference.
        let new_ctx = PyDict::new(py);
        for (k, v) in ctx.iter() {
            new_ctx.set_item(k, v)?;
        }
        Ok(Some(new_ctx))
    } else {
        Ok(None)
    }
}

// We'll fetch `id` function once and reuse it.
static ID_FUNC: OnceCell<Py<PyAny>> = OnceCell::new();

fn get_id_func(py: Python) -> PyResult<&PyAny> {
    ID_FUNC
        .get_or_try_init(|| py.eval("id", None, None).map(|f| f.into()))
        .map(|f| f.as_ref(py))
}

#[allow(clippy::too_many_arguments)]
fn do_visit_collection(
    py: Python<'_>,
    expr: &PyAny,
    visit_fn: &PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&PyDict>,
    remove_annotations: bool,
    seen: Option<&PySet>,
) -> PyResult<PyObject> {
    // If max_depth == 0, just call visit_fn and return early
    if max_depth == 0 {
        let visited = if let Some(ctx) = context {
            visit_fn.call1(py, (expr, ctx))?
        } else {
            visit_fn.call1(py, (expr,))?
        };
        return if return_data {
            Ok(visited.to_object(py))
        } else {
            Ok(py.None())
        };
    }

    // call visit_fn first
    let visited = if let Some(ctx) = context {
        visit_fn.call1(py, (expr, ctx))?
    } else {
        visit_fn.call1(py, (expr,))?
    };

    let current_expr = if return_data {
        visited.as_ref(py)
    } else {
        expr
    };

    // Check max_depth if positive
    if max_depth > 0 && max_depth == 1 {
        // Next level is 0-depth, so no recursion on children
        // Just return now if it's not a collection or if no recursion needed.
        // But we already called visit_fn, so just return current_expr
        // If we must return data, return visited. Otherwise None.
        if !current_expr.is_instance_of::<PyList>() && !current_expr.is_instance_of::<PyDict>() {
            return if return_data {
                Ok(current_expr.to_object(py))
            } else {
                Ok(py.None())
            };
        }
        // else we continue - we know max_depth > 1 or we handle below
    }

    // Check seen if available
    if let Some(seen_set) = seen {
        let id_func = get_id_func(py)?;
        let obj_id = id_func.call1((current_expr,))?;
        if seen_set.contains(obj_id)? {
            return if return_data {
                Ok(current_expr.to_object(py))
            } else {
                Ok(py.None())
            };
        }
        seen_set.add(obj_id)?;
    }

    // Now handle collections
    if let Ok(list) = current_expr.downcast::<PyList>() {
        list::visit_list(
            py,
            list,
            visit_fn,
            return_data,
            max_depth,
            context,
            remove_annotations,
            seen,
        )
    } else if let Ok(dict_obj) = current_expr.downcast::<PyDict>() {
        dict::visit_dict(
            py,
            dict_obj,
            visit_fn,
            return_data,
            max_depth,
            context,
            remove_annotations,
            seen,
        )
    } else if return_data {
        Ok(current_expr.to_object(py))
    } else {
        Ok(py.None())
    }
}

#[pyfunction]
#[pyo3(signature = (
    expr,
    visit_fn,
    return_data = false,
    max_depth = -1,
    context = None,
    remove_annotations = false,
    _seen = None
))]
#[allow(clippy::too_many_arguments)]
fn visit_collection(
    py: Python<'_>,
    expr: &PyAny,
    visit_fn: PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&PyDict>,
    remove_annotations: bool,
    _seen: Option<&PySet>,
) -> PyResult<PyObject> {
    do_visit_collection(
        py,
        expr,
        &visit_fn,
        return_data,
        max_depth,
        context,
        remove_annotations,
        _seen,
    )
}

#[pymodule]
fn visit_collection_rs(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Pre-load the `id` function and store it in ID_FUNC
    let _ = get_id_func(py)?; // load once

    m.add_function(wrap_pyfunction!(visit_collection, m)?)?;
    Ok(())
}
