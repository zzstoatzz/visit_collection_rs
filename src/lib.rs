use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PySet};

mod dict;
mod list;

// Add this near the top of lib.rs, after the imports and before the modules
pub(crate) fn copy_context<'py>(
    py: Python<'py>,
    context: Option<&PyDict>,
) -> PyResult<Option<&'py PyDict>> {
    Ok(if let Some(ctx) = context {
        let new_ctx = PyDict::new(py);
        for (k, v) in ctx.iter() {
            new_ctx.set_item(k, v)?;
        }
        Some(new_ctx)
    } else {
        None
    })
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
    // First, call visit_fn on the current object
    let visited = if let Some(ctx) = context {
        visit_fn.call1(py, (expr, ctx))?
    } else {
        visit_fn.call1(py, (expr,))?
    };

    // Update expr if return_data is true
    let current_expr = if return_data {
        visited.as_ref(py)
    } else {
        expr
    };

    // Check max_depth and seen
    if max_depth == 0 {
        return if return_data {
            Ok(current_expr.to_object(py))
        } else {
            Ok(py.None())
        };
    }

    if let Some(seen_set) = seen {
        let id_fn = py.eval("id", None, None)?;
        let obj_id = id_fn.call1((current_expr,))?;
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
fn visit_collection_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visit_collection, m)?)?;
    Ok(())
}
