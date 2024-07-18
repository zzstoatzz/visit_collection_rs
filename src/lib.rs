use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PySet};

mod list;
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
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn visit_collection<'py>(
    py: Python<'py>,
    expr: &Bound<'py, PyAny>,
    visit_fn: PyObject,
    return_data: bool,
    max_depth: i64,
    context: Option<&Bound<'py, PyDict>>,
    remove_annotations: bool,
    _seen: Option<&Bound<'py, PySet>>,
) -> PyResult<PyObject> {
    let _ = (remove_annotations, _seen); // TODO: use these

    if let Ok(list) = expr.downcast::<PyList>() {
        list::visit_list(py, list, &visit_fn, return_data, max_depth, context)
    } else if max_depth != 0 {
        if let Some(ctx) = context {
            visit_fn.call1(py, (expr, ctx))
        } else {
            visit_fn.call1(py, (expr,))
        }
    } else if return_data {
        Ok(expr.to_object(py))
    } else {
        Ok(py.None())
    }
}

#[pymodule]
fn visit_collection_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(visit_collection, m)?)?;
    Ok(())
}
